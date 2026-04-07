import hou
import sys


def probe_node_io(node: hou.Node, context_msg=""):
    print(f"[{node.name()}] {context_msg}")
    try:
        print(f"  Input Names  : {node.inputNames()}")
    except Exception as e:
        print(f"  Input Error  : {e}")
    try:
        print(f"  Output Names : {node.outputNames()}")
    except Exception as e:
        print(f"  Output Error : {e}")
    print("-" * 40)


def main():
    print("=== Houdini Ramen Probe 2: Dynamic Ports ===")

    parent = hou.node("/obj").createNode("geo", "probe_temp_geo2")

    try:
        vopnet = parent.createNode("attribvop", "probe_vop")

        print("\n--- 1. Variadic Inputs (Add VOP) ---")
        add_vop = vopnet.createNode("add", "probe_add")
        probe_node_io(add_vop, "(Initial State)")

        const1 = vopnet.createNode("constant")
        const2 = vopnet.createNode("constant")
        const3 = vopnet.createNode("constant")

        add_vop.setInput(0, const1)
        add_vop.setInput(1, const2)
        add_vop.setInput(2, const3)
        probe_node_io(add_vop, "(After connecting 3 inputs)")

        print("\n--- 2. Dynamic Output Name (Bind VOP) ---")
        bind_vop = vopnet.createNode("bind", "probe_bind")
        probe_node_io(bind_vop, "(Initial State)")

        bind_vop.parm("parmname").set("my_custom_attr")
        probe_node_io(bind_vop, "(After changing 'parmname' to 'my_custom_attr')")

    except Exception as e:
        print(f"Probe Failed: {e}", file=sys.stderr)
        sys.exit(1)
    finally:
        parent.destroy()
        print("=== Probe Complete ===")


if __name__ == "__main__":
    main()
