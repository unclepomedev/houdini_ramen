import hou


def check_builtin_inner_nodes():
    obj = hou.node("/obj")
    sop_parent = obj.node("geo1")
    created_sop_parent = False
    if not sop_parent:
        sop_parent = obj.createNode("geo", "geo1")
        created_sop_parent = True
    test_cases = [
        ("Sop", "solver"),
        ("Sop", "subnet"),
        ("Sop", "vopnet"),
        ("Object", "geo"),
        ("Dop", "dopnet"),
        ("Chop", "ch"),
    ]

    print("--- Testing builtin inner nodes ---")

    # This script tests partial categories.
    try:
        for cat_name, node_type_name in test_cases:
            try:
                if cat_name == "Sop":
                    parent_path = sop_parent.path()
                elif cat_name == "Chop":
                    parent_path = "/ch"
                elif cat_name == "Cop2":
                    parent_path = "/img"
                elif cat_name == "Dop":
                    parent_path = "/obj"
                elif cat_name == "Top":
                    parent_path = "/tasks"
                elif cat_name == "Vop":
                    parent_path = "/obj/geo1/vopnet1"
                    geo = hou.node("/obj").createNode("geo", "geo1")
                    geo.createNode("vopnet", "vopnet1")
                else:
                    parent_path = "/obj"

                parent = hou.node(parent_path)
                if not parent:
                    print(
                        f"[{cat_name}/{node_type_name}] Skipped: Parent network not found."
                    )
                    continue

                temp_node = None
                try:
                    temp_node = parent.createNode(node_type_name)
                    children = temp_node.children()
                    child_names = [child.name() for child in children]
                    print(
                        f"[{cat_name}/{node_type_name}] Built-in nodes: {child_names}"
                    )
                finally:
                    if temp_node:
                        temp_node.destroy()

            except Exception as e:
                print(f"[{cat_name}/{node_type_name}] Failed: {e}")
    finally:
        if created_sop_parent and sop_parent:
            sop_parent.destroy()


if __name__ == "__main__":
    check_builtin_inner_nodes()
