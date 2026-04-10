import json
import logging
from pathlib import Path
import hou

logging.basicConfig(level=logging.INFO, format="%(levelname)s - %(message)s")
logger = logging.getLogger(__name__)


class VopContextManager:
    def __init__(self):
        self.roots = []
        self.contexts = {}
        self._setup_contexts()

    def _setup_contexts(self):
        try:
            geo = hou.node("/obj").createNode("geo", "temp_geo", run_init_scripts=False)
            self.roots.append(geo)
            self.contexts["attribvop (SOP)"] = geo.createNode(
                "attribvop", run_init_scripts=True
            )
            self.contexts["volumevop (SOP)"] = geo.createNode(
                "volumevop", run_init_scripts=True
            )
        except Exception as e:
            logger.error(f"SOP context setup failed: {e}")

        try:
            mat_node = hou.node("/mat")
            if mat_node is not None:
                self.contexts["matnet (Shading)"] = mat_node
            else:
                logger.warning(
                    "hou.node('/mat') returned None, skipping matnet context"
                )
        except Exception as e:
            logger.error(f"MAT context setup failed: {e}")

        try:
            cop_net = hou.node("/obj").createNode(
                "cop2net", "temp_cop", run_init_scripts=False
            )
            self.roots.append(cop_net)
            self.contexts["vopcop2filter (COP)"] = cop_net.createNode(
                "vopcop2filter", run_init_scripts=True
            )
        except Exception as e:
            logger.error(f"COP context setup failed: {e}")

        try:
            chop_net = hou.node("/obj").createNode(
                "chopnet", "temp_chop", run_init_scripts=False
            )
            self.roots.append(chop_net)
            self.contexts["vopchop (CHOP)"] = chop_net.createNode(
                "vopchop", run_init_scripts=True
            )
        except Exception as e:
            logger.error(f"CHOP context setup failed: {e}")

        try:
            dop_net = hou.node("/obj").createNode(
                "dopnet", "temp_dop", run_init_scripts=False
            )
            self.roots.append(dop_net)
            self.contexts["popvop (DOP)"] = dop_net.createNode(
                "popvop", run_init_scripts=True
            )
        except Exception as e:
            logger.error(f"DOP context setup failed: {e}")

        try:
            stage = hou.node("/stage")
            lop_mat = stage.createNode(
                "materialbuilder", "temp_lop_mat", run_init_scripts=True
            )
            self.roots.append(lop_mat)
            self.contexts["materialbuilder (LOP)"] = lop_mat
        except Exception as e:
            logger.error(f"LOP context setup failed: {e}")

        try:
            plain_vop = hou.node("/obj").createNode(
                "vopnet", "temp_plain_vop", run_init_scripts=False
            )
            self.roots.append(plain_vop)
            self.contexts["plain_vopnet"] = plain_vop
        except Exception as e:
            logger.error(f"Plain VOP context setup failed: {e}")

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc, tb):
        self.cleanup()
        return False

    def cleanup(self):
        for root in self.roots:
            try:
                root.destroy()
            except Exception:
                logger.debug("failed to destroy root %s", root, exc_info=True)
        self.roots.clear()
        self.contexts.clear()


def probe_vop_contexts():
    cat = hou.nodeTypeCategories().get("Vop")
    if not cat:
        logger.error("VOP category not found.")
        return

    manager = VopContextManager()

    report = {
        "summary": {"total": len(cat.nodeTypes()), "success": 0, "failed_all": 0},
        "context_wins": {ctx_name: 0 for ctx_name in manager.contexts.keys()},
        "node_mapping": {},
        "failed_nodes": [],
    }

    test_order = [
        "attribvop (SOP)",
        "matnet (Shading)",
        "materialbuilder (LOP)",
        "vopcop2filter (COP)",
        "vopchop (CHOP)",
        "popvop (DOP)",
        "volumevop (SOP)",
        "plain_vopnet",
    ]

    try:
        for node_type_name, _node_type in cat.nodeTypes().items():
            success = False

            for ctx_name in test_order:
                if ctx_name not in manager.contexts:
                    continue
                parent = manager.contexts[ctx_name]

                temp_node = None
                try:
                    temp_node = parent.createNode(
                        node_type_name, run_init_scripts=False
                    )

                    if hasattr(temp_node, "outputLabels"):
                        _ = temp_node.outputLabels()

                    report["context_wins"][ctx_name] += 1
                    report["node_mapping"][node_type_name] = ctx_name
                    report["summary"]["success"] += 1
                    success = True
                    break

                except hou.OperationFailed:
                    logger.debug(
                        "OperationFailed for '%s' in context '%s'",
                        node_type_name,
                        ctx_name,
                        exc_info=True,
                    )
                except Exception:
                    logger.debug(
                        "Unexpected error for '%s' in context '%s'",
                        node_type_name,
                        ctx_name,
                        exc_info=True,
                    )
                finally:
                    if temp_node:
                        try:
                            temp_node.destroy()
                        except Exception:
                            logger.debug(
                                "failed to destroy temp node '%s' in context '%s'",
                                node_type_name,
                                ctx_name,
                                exc_info=True,
                            )

            if not success:
                report["summary"]["failed_all"] += 1
                report["failed_nodes"].append(node_type_name)

    finally:
        manager.cleanup()

    out_path = Path("vop_context_mapping.json")
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)

    logger.info(f"Probe completed. Summary:\n{json.dumps(report['summary'], indent=2)}")
    logger.info(f"Context breakdown:\n{json.dumps(report['context_wins'], indent=2)}")


if __name__ == "__main__":
    probe_vop_contexts()
