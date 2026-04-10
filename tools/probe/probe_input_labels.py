import re
import sys

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


def normalize_label(label: str) -> str:
    s = re.sub(r"[^a-zA-Z0-9]", "_", label.strip())
    s = re.sub(r"_+", "_", s).strip("_")
    return s.lower()


def main():
    temp_manager = TempNodeManager()
    issue_nodes = []

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
                labels = list(temp_node.inputLabels())

                issues = []
                if len(labels) < max_in:
                    issues.append(
                        f"Missing labels: max_in={max_in}, but got {len(labels)} labels"
                    )

                normalized_labels = []
                for i, label in enumerate(labels):
                    if not label.strip():
                        issues.append(f"Empty label at index {i}")
                    else:
                        norm = normalize_label(label)
                        if not norm:
                            issues.append(
                                f"Label '{label}' at index {i} becomes empty after normalization"
                            )
                        elif norm in normalized_labels:
                            issues.append(
                                f"Duplicate normalized label '{norm}' at index {i} (original: '{label}')"
                            )
                        normalized_labels.append(norm)

                if issues:
                    issue_nodes.append(
                        {
                            "category": cat_name,
                            "type": node_type_name,
                            "issues": issues,
                            "raw_labels": labels,
                        }
                    )

            except Exception as e:
                issue_nodes.append(
                    {
                        "category": cat_name,
                        "type": node_type_name,
                        "issues": [f"Exception: {e}"],
                        "raw_labels": [],
                    }
                )
            finally:
                try:
                    temp_node.destroy()
                except Exception:
                    pass

    temp_manager.cleanup()

    print(f"Total problematic node types found: {len(issue_nodes)}\n")
    for info in issue_nodes:
        print(f"[{info['category']} / {info['type']}]")
        for issue in info["issues"]:
            print(f"  - {issue}")
        print(f"  Raw Labels: {info['raw_labels']}")
        print("-" * 60)


if __name__ == "__main__":
    main()
