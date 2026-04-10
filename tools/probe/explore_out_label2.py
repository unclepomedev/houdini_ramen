import json
import logging
from pathlib import Path
import hou

logging.basicConfig(level=logging.INFO, format="%(levelname)s - %(message)s")
logger = logging.getLogger(__name__)


class TempNodeManager:
    def __init__(self):
        self.parents = {}
        self._strategies = {
            "Sop": ("/obj", "geo", "temp_sop_parent"),
            "Chop": ("/ch", "ch", "temp_chop_parent"),
            "Cop2": ("/img", "img", "temp_cop_parent"),
            "Dop": ("/obj", "dopnet", "temp_dop_parent"),
            "Top": ("/tasks", "topnet", "temp_top_parent"),
            "Vop": ("/obj", "vopnet", "temp_vop_parent"),
            "Object": (None, None, "/obj"),
            "Lop": (None, None, "/stage"),
            "Driver": (None, None, "/out"),
            "Shop": ("/shop", "shop", "temp_shop_parent"),
        }

    def get_parent(self, cat_name: str):
        if cat_name in self.parents:
            return self.parents[cat_name]

        parent = None
        try:
            if cat_name in self._strategies:
                root_path, node_type, target_path = self._strategies[cat_name]
                if node_type:
                    root = hou.node(root_path)
                    if root:
                        parent = root.createNode(
                            node_type, target_path, run_init_scripts=False
                        )
                else:
                    parent = hou.node(target_path)
        except Exception as e:
            logger.debug(f"Failed to create temp parent for '{cat_name}': {e}")

        self.parents[cat_name] = parent
        return parent

    def cleanup(self):
        for cat_name, node in self.parents.items():
            if node and cat_name in self._strategies and self._strategies[cat_name][1]:
                try:
                    node.destroy()
                except Exception as e:
                    logger.debug(f"Failed to destroy temp parent for '{cat_name}': {e}")
        self.parents.clear()


def probe_all_nodes():
    manager = TempNodeManager()
    report = {
        "summary": {},
        "failures_instantiation": {},
        "failures_io_read": {},
        "success_samples": {},
    }

    try:
        categories = hou.nodeTypeCategories()
        for cat_name, cat in sorted(categories.items()):
            logger.info(f"Probing category: {cat_name}")
            parent = manager.get_parent(cat_name)

            cat_stats = {
                "total": len(cat.nodeTypes()),
                "success": 0,
                "instantiation_failed": 0,
                "io_read_failed": 0,
            }

            report["failures_instantiation"][cat_name] = []
            report["failures_io_read"][cat_name] = []

            for node_type_name, _node_type in cat.nodeTypes().items():
                if not parent:
                    cat_stats["instantiation_failed"] += 1
                    report["failures_instantiation"][cat_name].append(
                        {
                            "node": node_type_name,
                            "reason": "No parent context available",
                        }
                    )
                    continue

                temp_node = None
                try:
                    temp_node = parent.createNode(
                        node_type_name, run_init_scripts=False
                    )
                except Exception as e:
                    cat_stats["instantiation_failed"] += 1
                    report["failures_instantiation"][cat_name].append(
                        {"node": node_type_name, "reason": f"{type(e).__name__}: {e}"}
                    )
                    continue

                try:
                    out_labels = (
                        temp_node.outputLabels()
                        if hasattr(temp_node, "outputLabels")
                        else ()
                    )
                    out_names = (
                        temp_node.outputNames()
                        if hasattr(temp_node, "outputNames")
                        else ()
                    )

                    cat_stats["success"] += 1

                    if cat_name not in report["success_samples"]:
                        report["success_samples"][cat_name] = {
                            "node": node_type_name,
                            "outputLabels": list(out_labels),
                            "outputNames": list(out_names),
                        }

                except Exception as e:
                    cat_stats["io_read_failed"] += 1
                    report["failures_io_read"][cat_name].append(
                        {"node": node_type_name, "reason": f"{type(e).__name__}: {e}"}
                    )
                finally:
                    try:
                        temp_node.destroy()
                    except Exception as e:
                        logger.debug(
                            f"Failed to destroy temp node '{node_type_name}' in '{cat_name}': {e}"
                        )

            report["summary"][cat_name] = cat_stats

    finally:
        manager.cleanup()

    out_path = Path("probe_io_report.json")
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=2)
    logger.info(f"Probe completed. Report saved to {out_path.absolute()}")


if __name__ == "__main__":
    probe_all_nodes()
