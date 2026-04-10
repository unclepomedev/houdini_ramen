import re
import sys
from collections import Counter

import hou

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


def normalize_name(name: str) -> str:
    s = re.sub(r"[^a-zA-Z0-9]", "_", name.strip())
    s = re.sub(r"_+", "_", s).strip("_")
    return s.lower()


def main():
    temp_manager = TempNodeManager()

    stats = {
        "total_nodes_checked": 0,
        "has_input_names": 0,
        "no_input_names": 0,
        "infinite_inputs_9999": 0,
    }

    issues = []

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

            stats["total_nodes_checked"] += 1
            if max_in >= 999:
                stats["infinite_inputs_9999"] += 1

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
                continue

            try:
                if not hasattr(temp_node, "inputNames"):
                    stats["no_input_names"] += 1
                    continue

                stats["has_input_names"] += 1

                try:
                    raw_names = list(temp_node.inputNames())
                except Exception as e:
                    issues.append(
                        f"[{cat_name} / {node_type_name}] Failed to call inputNames(): {e}"
                    )
                    continue

                node_issues = []

                empty_count = sum(1 for n in raw_names if not n.strip())
                if empty_count > 0:
                    node_issues.append(f"Contains {empty_count} empty names")

                normalized = [normalize_name(n) for n in raw_names if n.strip()]
                counts = Counter(normalized)
                duplicates = {k: v for k, v in counts.items() if v > 1}
                if duplicates:
                    node_issues.append(f"Duplicate normalized names: {duplicates}")

                if node_issues:
                    issues.append(
                        {
                            "category": cat_name,
                            "type": node_type_name,
                            "issues": node_issues,
                            "raw_names": raw_names,
                        }
                    )

            except Exception as e:
                issues.append(f"[{cat_name} / {node_type_name}] Unexpected error: {e}")
            finally:
                try:
                    temp_node.destroy()
                except Exception:
                    pass

    temp_manager.cleanup()

    print("=== SUMMARY ===")
    print(f"Total Nodes Checked (with inputs > 0): {stats['total_nodes_checked']}")
    print(f"Nodes with infinite inputs (>= 999): {stats['infinite_inputs_9999']}")
    print(f"Nodes with 'inputNames()' method: {stats['has_input_names']}")
    print(f"Nodes WITHOUT 'inputNames()' method: {stats['no_input_names']}")
    print(
        f"Nodes with problematic inputNames: {len([i for i in issues if isinstance(i, dict)])}"
    )

    print("\n=== PROBLEMATIC INPUT NAMES ===")
    for issue in issues:
        if isinstance(issue, str):
            print(issue)
        else:
            print(f"[{issue['category']} / {issue['type']}]")
            for i in issue["issues"]:
                print(f"  - {i}")
            print(f"  Raw Names: {issue['raw_names']}")
            print("-" * 60)


if __name__ == "__main__":
    main()
