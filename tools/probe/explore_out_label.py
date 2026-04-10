import hou


def probe_outputs():
    obj = hou.node("/obj")
    geo = obj.createNode("geo", "temp_geo_for_probe", run_init_scripts=False)

    try:
        attribvop = geo.createNode("attribvop", run_init_scripts=True)

        test_nodes = {
            "geometryvopglobal1": attribvop.node("geometryvopglobal1"),
            "add1": attribvop.createNode("add"),
            "bind1": attribvop.createNode("bind"),
        }

        for name, node in test_nodes.items():
            if not node:
                continue

            print(f"\n=== {node.type().name()} ===")

            if hasattr(node, "outputLabels"):
                print(f"outputLabels: {node.outputLabels()}")

            if hasattr(node, "outputNames"):
                print(f"outputNames: {node.outputNames()}")

            if hasattr(node, "outputDataTypes"):
                print(f"outputDataTypes: {node.outputDataTypes()}")

            output_methods = [m for m in dir(node) if "output" in m.lower()]
            print(f"output_methods: {output_methods}")

    except Exception as e:
        print(f"Error: {e}")

    finally:
        geo.destroy()


if __name__ == "__main__":
    probe_outputs()
