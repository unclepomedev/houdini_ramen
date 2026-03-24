import hou


def main():
    category = hou.sopNodeTypeCategory()
    node_type = category.nodeTypes().get("copytopoints")

    if not node_type:
        print("Node type 'copytopoints' not found.")
        return

    print("==================================================")
    print(f" Inspecting hou.NodeType: {node_type.name()}")
    print("==================================================")
    nt_methods = [attr for attr in dir(node_type) if "input" in attr.lower()]
    print("Methods containing 'input':")
    for m in nt_methods:
        print(f"  - {m}")

    print("\n==================================================")
    print(" Inspecting hou.Node (Instance)")
    print("==================================================")
    temp_geo = hou.node("/obj").createNode("geo", "temp_api_explorer")
    try:
        node_instance = temp_geo.createNode("copytopoints")

        n_methods = [attr for attr in dir(node_instance) if "input" in attr.lower()]
        print("Methods containing 'input':")
        for m in n_methods:
            print(f"  - {m}")

        if hasattr(node_instance, "inputLabels"):
            print("\nActual input labels from instance:")
            print(f"  {node_instance.inputLabels()}")
    finally:
        temp_geo.destroy()


if __name__ == "__main__":
    main()
