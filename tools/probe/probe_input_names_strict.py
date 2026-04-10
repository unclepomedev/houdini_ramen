import sys
import hou
from collections import defaultdict

IGNORED_CATEGORIES = {
    "Cop",
    "Shop",
    "Data",
    "Manager",
    "Director",
    "VopNet",
    "CopNet",
    "ChopNet",
    "TopNet",
}


class TempNodeManager:
    def __init__(self):
        self.parents: dict[str, list[hou.Node]] = {}
        self._strategies = {
            "Sop": ("/obj", "geo", "temp_sop_parent"),
            "Chop": ("/ch", "ch", "temp_chop_parent"),
            "Cop2": ("/img", "img", "temp_cop_parent"),
            "Dop": ("/obj", "dopnet", "temp_dop_parent"),
            "Top": ("/tasks", "topnet", "temp_top_parent"),
        }
        self._direct_roots = {
            "Object": "/obj",
            "Lop": "/stage",
            "Driver": "/out",
        }

    def get_parents(self, cat_name: str) -> list[hou.Node]:
        if cat_name in self.parents:
            return self.parents[cat_name]
        parents = []
        try:
            if cat_name == "Vop":
                root = hou.node("/obj")
                if root:
                    geo = root.createNode("geo", "temp_vop_geo", run_init_scripts=False)
                    parents.append(
                        geo.createNode(
                            "attribvop", "temp_attribvop", run_init_scripts=True
                        )
                    )
                mat = hou.node("/mat")
                if mat:
                    parents.append(mat)
            elif cat_name in self._strategies:
                root_path, node_type, node_name = self._strategies[cat_name]
                root = hou.node(root_path)
                if root:
                    parents.append(
                        root.createNode(node_type, node_name, run_init_scripts=False)
                    )
            elif cat_name in self._direct_roots:
                node = hou.node(self._direct_roots[cat_name])
                if node:
                    parents.append(node)
        except Exception:
            pass
        self.parents[cat_name] = parents
        return parents

    def cleanup(self):
        for cat_name, node_list in self.parents.items():
            for node in node_list:
                if not node:
                    continue
                try:
                    if cat_name == "Vop" and node.type().name() == "attribvop":
                        node.parent().destroy()
                    elif cat_name not in self._direct_roots and not (
                        cat_name == "Vop" and node.path() == "/mat"
                    ):
                        node.destroy()
                except Exception:
                    pass
        self.parents.clear()


def main():
    try:
        import logging

        hou.logging.logToConsole(False)
    except AttributeError:
        pass

    temp_manager = TempNodeManager()

    stats = {
        "checked": 0,
        "failed_to_create": 0,
        "failed_to_read_input_names": 0,
        "no_names_returned": 0,
        "has_valid_names": 0,
    }

    samples = defaultdict(list)
    failed_nodes = []

    for cat_name, cat in sorted(hou.nodeTypeCategories().items()):
        if cat_name in IGNORED_CATEGORIES:
            continue

        parents = temp_manager.get_parents(cat_name)
        if not parents:
            continue

        for node_type_name, node_type in cat.nodeTypes().items():
            max_in = node_type.maxNumInputs()
            if max_in <= 0:
                continue

            stats["checked"] += 1

            temp_node = None
            for parent in parents:
                try:
                    temp_node = parent.createNode(
                        node_type_name, run_init_scripts=False
                    )
                    break
                except Exception:
                    pass

            if not temp_node:
                stats["failed_to_create"] += 1
                failed_nodes.append(f"{cat_name} / {node_type_name}")
                continue

            try:
                raw_names = list(temp_node.inputNames())

                if len(raw_names) == 0:
                    stats["no_names_returned"] += 1
                else:
                    stats["has_valid_names"] += 1
                    if len(samples[cat_name]) < 3:
                        samples[cat_name].append((node_type_name, raw_names))

            except Exception:
                stats["failed_to_read_input_names"] += 1
                failed_nodes.append(
                    f"{cat_name} / {node_type_name} (Exception on inputNames)"
                )
            finally:
                try:
                    temp_node.destroy()
                except Exception:
                    pass

    temp_manager.cleanup()

    print("=== STRICT VALIDATION SUMMARY ===")
    print(f"Total Nodes with Inputs (>0)    : {stats['checked']}")
    print(f"Failed to Instantiate (Skipped) : {stats['failed_to_create']}")
    print(f"Failed to Read inputNames()     : {stats['failed_to_read_input_names']}")
    print(f"Returned EMPTY inputNames ()    : {stats['no_names_returned']}  <-- !!!")
    print(f"Successfully returned names     : {stats['has_valid_names']}")

    print("\n=== SAMPLE VALID INPUT NAMES BY CATEGORY ===")
    for cat_name, sample_list in samples.items():
        print(f"[{cat_name}]")
        for node_name, names in sample_list:
            print(f"  - {node_name}: {names[:5]} {'...' if len(names) > 5 else ''}")

    print("\n=== FAILED TO INSTANTIATE (First 10) ===")
    for f in failed_nodes[:10]:
        print(f"  - {f}")
    if len(failed_nodes) > 10:
        print(f"  ... and {len(failed_nodes) - 10} more.")


if __name__ == "__main__":
    main()
