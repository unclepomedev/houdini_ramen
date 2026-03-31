import hou


def probe_various_containers():
    test_cases = [
        ("/obj", "geo", "geo_container"),
        ("/obj", "dopnet", "dop_network"),
        ("/ch", "ch", "chop_network"),
        ("/obj/geo_container", "subnet", "sop_subnet"),
        ("/obj/geo_container", "solver", "sop_solver"),
        ("/obj/geo_container", "vopnet", "sop_vopnet"),
        ("/obj/geo_container", "vellumsolver", "vellum"),
    ]

    print("=== Houdini Container Architecture Probe ===")

    obj = hou.node("/obj")
    if obj is None:
        print("  [Error] Parent '/obj' not found.")
        return
    geo = obj.node("geo_container")
    created_geo = False
    if not geo:
        geo = obj.createNode("geo", "geo_container")
        created_geo = True

    for parent_path, node_type, node_name in test_cases:
        print(f"\n--- Node Type: {node_type} ---")
        parent = hou.node(parent_path)
        if not parent:
            print(f"  [Skip] Parent '{parent_path}' not found.")
            continue

        node = None
        try:
            node = parent.createNode(node_type, node_name)
            direct_children = [c.name() for c in node.children()]
            print(f"  [Direct Children] ({len(direct_children)}): {direct_children}")

            rep = node.representativeNode()
            if rep:
                if rep == node:
                    print("  [Representative] Points to ITSELF.")
                else:
                    rep_rel_path = rep.path().replace(node.path() + "/", "")
                    print(f"  [Representative] Points to: {rep_rel_path}")

                    rep_children = [c.name() for c in rep.children()]
                    print(f"  [Rep's Children] ({len(rep_children)}): {rep_children}")
            else:
                print("  [Representative] None")

        except Exception as e:
            print(f"  [Error] Failed to create or probe '{node_type}': {e}")
        finally:
            if node:
                try:
                    node.destroy()
                except Exception as destroy_error:
                    print(
                        f"  [Warn] Failed to destroy '{node.path()}': {destroy_error}"
                    )

    if created_geo and geo:
        try:
            geo.destroy()
        except Exception as destroy_error:
            print(f"  [Warn] Failed to destroy '/obj/geo_container': {destroy_error}")


if __name__ == "__main__":
    probe_various_containers()
