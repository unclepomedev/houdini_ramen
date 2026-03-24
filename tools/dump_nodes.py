import argparse
import json
import logging
import os
import sys
import tempfile
from dataclasses import asdict, dataclass, field, is_dataclass
from pathlib import Path
from typing import Any

import hou

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)

LOG_FILE = "houdini_warnings.log"


@dataclass
class ParmInfo:
    name: str
    type: str
    default: Any = None


@dataclass
class NodeInfo:
    min_inputs: int
    max_inputs: int
    input_labels: list[str] = field(default_factory=list)
    parms: list[ParmInfo] = field(default_factory=list)


def _exclude_none_factory(data: list[tuple[str, Any]]) -> dict[str, Any]:
    return {k: v for k, v in data if v is not None}


class HoudiniJSONEncoder(json.JSONEncoder):
    """encoder that converts data classes and Houdini-specific tuple-like objects to JSON"""

    def default(self, obj: Any) -> Any:
        if is_dataclass(obj):
            return asdict(obj, dict_factory=_exclude_none_factory)
        if isinstance(obj, tuple):
            return list(obj)
        return super().default(obj)


class AtomicJSONWriter:
    """file I/O to prevent incomplete file writing"""

    def __init__(self, output_path: Path):
        self.output_path = output_path
        self.output_path.parent.mkdir(parents=True, exist_ok=True)

    def write(self, data: Any) -> None:
        fd, tmp_name = tempfile.mkstemp(
            prefix=f".{self.output_path.name}.",
            suffix=".tmp",
            dir=self.output_path.parent,
        )
        tmp_path = Path(tmp_name)

        try:
            with os.fdopen(fd, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2, cls=HoudiniJSONEncoder)
                f.write("\n")
                f.flush()
                os.fsync(f.fileno())
            os.replace(tmp_path, self.output_path)
        except Exception as e:
            logger.error(f"Failed to write JSON: {e}")
            try:
                tmp_path.unlink(missing_ok=True)
            except OSError as cleanup_error:
                logger.warning(
                    f"Failed to clean up temp file: {tmp_path}", exc_info=cleanup_error
                )
            raise


class HoudiniLogContext:
    """
    Houdini native warnings/errors are redirected to a log file instead of stdout/stderr.
    In debug mode, it leaves the standard output streams untouched.
    """

    def __init__(self, debug: bool, log_file: str = LOG_FILE):
        self.debug = debug
        self.log_file = log_file

    def __enter__(self):
        if not self.debug:
            sys.stdout.flush()
            sys.stderr.flush()
            self.log_fd = os.open(self.log_file, os.O_WRONLY | os.O_CREAT | os.O_TRUNC)
            self.save_stdout = os.dup(1)
            self.save_stderr = os.dup(2)
            os.dup2(self.log_fd, 1)
            os.dup2(self.log_fd, 2)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if not self.debug:
            sys.stdout.flush()
            sys.stderr.flush()
            os.dup2(self.save_stdout, 1)
            os.dup2(self.save_stderr, 2)
            os.close(self.save_stdout)
            os.close(self.save_stderr)
            os.close(self.log_fd)


class TempNodeManager:
    def __init__(self):
        self.parents: dict[str, hou.Node] = {}
        self._strategies = {
            "Sop": ("/obj", "geo", "temp_sop_parent"),
            "Chop": ("/ch", "ch", "temp_chop_parent"),
            "Cop2": ("/img", "img", "temp_cop_parent"),
            "Dop": ("/obj", "dopnet", "temp_dop_parent"),
            "Top": ("/tasks", "topnet", "temp_top_parent"),
            "Vop": ("/obj", "vopnet", "temp_vop_parent"),
        }
        self._direct_roots = {
            "Object": "/obj",
            "Lop": "/stage",
            "Driver": "/out",
        }

    def get_parent(self, cat_name: str):
        if cat_name in self.parents:
            return self.parents[cat_name]

        parent = None
        try:
            if cat_name in self._strategies:
                root_path, node_type, node_name = self._strategies[cat_name]
                root = hou.node(root_path)
                if root:
                    parent = root.createNode(
                        node_type, node_name, run_init_scripts=False
                    )
            elif cat_name in self._direct_roots:
                parent = hou.node(self._direct_roots[cat_name])
        except Exception as e:
            logger.debug(f"Failed to create temp parent for category '{cat_name}': {e}")

        self.parents[cat_name] = parent
        return parent

    def cleanup(self):
        for cat_name, node in self.parents.items():
            if node and cat_name in self._strategies:
                try:
                    node.destroy()
                except Exception as e:
                    logger.debug(
                        f"Failed to destroy temp parent for category '{cat_name}': {e}"
                    )
        self.parents.clear()


class HoudiniNodeExtractor:
    """traverses Houdini node information and converts it into a data model"""

    def __init__(self):
        self.temp_manager = TempNodeManager()

    @staticmethod
    def _get_default_value(pt: hou.ParmTemplate):
        if not hasattr(pt, "defaultValue"):
            return None
        try:
            return pt.defaultValue()
        except hou.OperationFailed as e:
            logger.debug(f"Failed to get default value for '{pt.name()}': {e}")
            return None

    def _extract_single_parm(self, pt: hou.ParmTemplate) -> ParmInfo:
        return ParmInfo(
            name=pt.name(), type=pt.type().name(), default=self._get_default_value(pt)
        )

    def _extract_parms_recursive(self, entries: tuple | list) -> list[ParmInfo]:
        parms = []
        for pt in entries:
            if isinstance(pt, hou.FolderParmTemplate):
                parms.extend(self._extract_parms_recursive(pt.parmTemplates()))
            else:
                parms.append(self._extract_single_parm(pt))
        return parms

    def _extract_input_labels(
        self, node_type: hou.NodeType, cat_name: str
    ) -> list[str]:
        """Temporarily instantiates a node to extract its input labels."""
        if node_type.maxNumInputs() <= 0:
            return []

        parent = self.temp_manager.get_parent(cat_name)
        if not parent:
            return []

        temp_node = None
        try:
            temp_node = parent.createNode(node_type.name(), run_init_scripts=False)
            if hasattr(temp_node, "inputLabels"):
                return list(temp_node.inputLabels())
            return []
        except Exception as e:
            logger.debug(f"Failed to extract labels for '{node_type.name()}': {e}")
            return []
        finally:
            if temp_node is not None:
                try:
                    temp_node.destroy()
                except Exception as destroy_error:
                    logger.debug(
                        f"Failed to destroy temp node '{node_type.name()}': {destroy_error}"
                    )

    def _extract_node_info(self, node_type: hou.NodeType, cat_name: str) -> NodeInfo:
        parms = []
        try:
            parms = self._extract_parms_recursive(
                node_type.parmTemplateGroup().entries()
            )
        except hou.PermissionError as e:
            logger.debug(
                f"Permission denied for parameters of '{node_type.name()}': {e}"
            )

        return NodeInfo(
            min_inputs=node_type.minNumInputs(),
            max_inputs=node_type.maxNumInputs(),
            input_labels=self._extract_input_labels(node_type, cat_name),
            parms=parms,
        )

    def extract_all_categories(self) -> dict[str, dict[str, NodeInfo]]:
        data: dict[str, dict[str, NodeInfo]] = {}
        try:
            for cat_name, cat in sorted(hou.nodeTypeCategories().items()):
                logger.info(f"Processing category: {cat_name}")
                cat_data: dict[str, NodeInfo] = {}

                for node_name, node_type in sorted(cat.nodeTypes().items()):
                    cat_data[node_name] = self._extract_node_info(node_type, cat_name)

                data[cat_name] = cat_data
        finally:
            self.temp_manager.cleanup()

        return data


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "output",
        nargs="?",
        default="node_api_dump.json",
        help="Output JSON filename (default: node_api_dump.json)",
    )
    parser.add_argument(
        "--debug",
        action="store_true",
        help="Enable debug logging and show Houdini native warnings in console.",
    )
    args = parser.parse_args()

    if args.debug:
        logger.setLevel(logging.DEBUG)

    output_path = Path(args.output)
    if not output_path.is_absolute():
        output_path = Path.cwd() / output_path

    try:
        logger.info("Starting node data extraction...")

        if not args.debug:
            logger.info(
                f"Houdini native warnings are redirected to '{LOG_FILE}'. Use --debug to print them to console."
            )

        with HoudiniLogContext(debug=args.debug):
            extractor = HoudiniNodeExtractor()
            node_data = extractor.extract_all_categories()

        logger.info(f"Writing data to {output_path}")
        writer = AtomicJSONWriter(output_path)
        writer.write(node_data)

        logger.info("Extraction completed successfully.")

    except Exception as e:
        logger.error(f"Fatal error during execution: {e}", exc_info=True)
        sys.exit(1)


if __name__ == "__main__":
    main()
