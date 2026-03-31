import hou


def probe_metadata():
    test_cases = [
        ("/obj", "geo", "geo_container"),
        ("/obj", "dopnet", "dop_network"),
        ("/obj/geo_container", "subnet", "sop_subnet"),
        ("/obj/geo_container", "solver", "sop_solver"),
        ("/obj/geo_container", "vellumsolver", "vellum"),
    ]

    print("=== Houdini Metadata Architecture Probe ===")

    obj = hou.node("/obj")
    geo = obj.node("geo_container")
    created_geo = False
    if not geo:
        geo = obj.createNode("geo", "geo_container")
        created_geo = True

    for parent_path, node_type, node_name in test_cases:
        print(f"\n--- Node Type: {node_type} ---")
        parent = hou.node(parent_path)
        if not parent:
            continue

        node = None
        try:
            node = parent.createNode(node_type, node_name)
            hda_def = node.type().definition()

            if hda_def:
                print("  [Type]: Digital Asset (HDA)")
                sections = hda_def.sections()

                if "DiveTarget" in sections:
                    dive_target = sections["DiveTarget"].contents().strip()
                    print(f"  [DiveTarget]: '{dive_target}'")
                else:
                    print("  [DiveTarget]: None")

                if "EditableNodes" in sections:
                    editable_nodes = sections["EditableNodes"].contents().strip()
                    print(f"  [EditableNodes]: '{editable_nodes}'")
                else:
                    print("  [EditableNodes]: None")
            else:
                print("  [Type]: Standard Built-in Node (Not an HDA)")

                indirect = node.indirectInputs()
                if indirect:
                    print(f"  [Indirect Inputs]: {[i.name() for i in indirect]}")
                else:
                    print("  [Indirect Inputs]: None")

        except Exception as e:
            print(f"  [Error] Failed to probe '{node_type}': {e}")
        finally:
            if node:
                node.destroy()

    if created_geo:
        geo.destroy()


if __name__ == "__main__":
    probe_metadata()
