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
    menu_items: list[str] = field(default_factory=list)
    menu_labels: list[str] = field(default_factory=list)


@dataclass
class NodeInfo:
    min_inputs: int
    max_inputs: int
    input_labels: list[str] = field(default_factory=list)
    parms: list[ParmInfo] = field(default_factory=list)
    builtin_inner_nodes: dict[str, str] = field(default_factory=dict)


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
    Houdini native warnings/errors are redirected to a log file.
    """

    def __init__(self, debug: bool, log_file: str = LOG_FILE):
        self.debug = debug
        self.log_file = log_file
        self.log_fd = None
        self.save_stdout = None
        self.save_stderr = None

    def __enter__(self):
        if not self.debug:
            sys.stdout.flush()
            sys.stderr.flush()
            try:
                self.log_fd = os.open(
                    self.log_file, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o644
                )
                self.save_stdout = os.dup(1)
                self.save_stderr = os.dup(2)
                os.dup2(self.log_fd, 1)
                os.dup2(self.log_fd, 2)
            except OSError:
                self._restore_fds()
                raise
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if not self.debug:
            sys.stdout.flush()
            sys.stderr.flush()
            self._restore_fds()

    def _restore_fds(self):
        if self.save_stdout is not None:
            try:
                os.dup2(self.save_stdout, 1)
                os.close(self.save_stdout)
            except OSError as e:
                try:
                    logger.error(f"Failed to restore stdout FD: {e}")
                except Exception:
                    # Logger crashes if stderr is broken. Ignore to prevent exception masking.
                    pass
            self.save_stdout = None

        if self.save_stderr is not None:
            try:
                os.dup2(self.save_stderr, 2)
                os.close(self.save_stderr)
            except OSError as e:
                try:
                    logger.error(f"Failed to restore stderr FD: {e}")
                except Exception:
                    pass
            self.save_stderr = None

        if self.log_fd is not None:
            try:
                os.close(self.log_fd)
            except OSError as e:
                try:
                    logger.error(f"Failed to close log file FD: {e}")
                except Exception:
                    pass
            self.log_fd = None


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
        info = ParmInfo(
            name=pt.name(), type=pt.type().name(), default=self._get_default_value(pt)
        )
        if hasattr(pt, "menuItems") and hasattr(pt, "menuLabels"):
            items = pt.menuItems()
            labels = pt.menuLabels()
            if items and labels and len(items) > 0 and len(items) == len(labels):
                info.menu_items = list(items)
                info.menu_labels = list(labels)
        return info

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
        """Temporarily instantiates a node to extract its input labels, with fallback for failures."""
        max_inputs = node_type.maxNumInputs()
        if max_inputs <= 0:
            return []

        parent = self.temp_manager.get_parent(cat_name)
        if not parent:
            return [""] * max_inputs if max_inputs < 128 else []

        temp_node = None
        try:
            temp_node = parent.createNode(node_type.name(), run_init_scripts=False)
            if hasattr(temp_node, "inputLabels"):
                return list(temp_node.inputLabels())
            return [""] * max_inputs if max_inputs < 128 else []
        except Exception as e:
            logger.debug(
                f"Input-label extraction failed for '{cat_name}/{node_type.name()}': {e}"
            )
            if 0 < max_inputs < 128:
                return [""] * max_inputs
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

        builtin_inner_nodes = self._extract_builtin_inner_nodes(node_type, cat_name)

        return NodeInfo(
            min_inputs=node_type.minNumInputs(),
            max_inputs=node_type.maxNumInputs(),
            input_labels=self._extract_input_labels(node_type, cat_name),
            parms=parms,
            builtin_inner_nodes=builtin_inner_nodes,
        )

    def _extract_builtin_inner_nodes(
            self, node_type: hou.NodeType, cat_name: str
    ) -> dict[str, str]:
        builtin_inner_nodes: dict[str, str] = {}
        parent = self.temp_manager.get_parent(cat_name)
        if not parent:
            return builtin_inner_nodes

        temp_node = None
        try:
            temp_node = parent.createNode(node_type.name())
            target_node = temp_node

            hda_def = temp_node.type().definition()
            if hda_def:
                sections = hda_def.sections()
                if "DiveTarget" in sections:
                    dive_path = sections["DiveTarget"].contents().strip()
                    if dive_path:
                        dive_node = temp_node.node(dive_path)
                        if dive_node:
                            target_node = dive_node

            for child in target_node.children():
                rel_path = child.path().replace(temp_node.path() + "/", "")
                builtin_inner_nodes[child.name()] = rel_path
        except Exception as e:
            logger.debug(
                f"Failed to extract inner nodes for {node_type.name()}: {e}"
            )
        finally:
            if temp_node is not None:
                try:
                    temp_node.destroy()
                except Exception:
                    pass

        return builtin_inner_nodes

    @staticmethod
    def _get_base_names(cat: hou.NodeTypeCategory) -> set[str]:
        names = set()
        for nt in cat.nodeTypes().values():
            comps = nt.nameComponents()
            ns, base = comps[1], comps[2]
            key = f"{ns}::{base}" if ns else base
            names.add(key)
        return names

    @staticmethod
    def _resolve_default_node_type(cat: hou.NodeTypeCategory, base_key: str, parent):
        if "::" in base_key:
            ns, base = base_key.split("::", 1)
        else:
            ns, base = "", base_key

        create_name = f"{ns}::{base}" if ns else base

        if parent:
            try:
                temp_node = parent.createNode(create_name, run_init_scripts=False)
                node_type = temp_node.type()
                temp_node.destroy()
                return node_type
            except Exception as e:
                logger.debug(
                    f"Failed to instantiate '{create_name}' for default version resolution: {e}"
                )

        node_types = cat.nodeTypes()
        if create_name in node_types:
            return node_types[create_name]

        candidates = [
            nt
            for _, nt in node_types.items()
            if nt.nameComponents()[1] == ns and nt.nameComponents()[2] == base
        ]

        if not candidates:
            logger.debug(
                f"Fallback failed: base key '{base_key}' not found in category"
            )
            return None

        def _version_key(nt):
            ver = nt.nameComponents()[3]
            if not ver:
                return []
            try:
                return [int(x) for x in ver.split(".")]
            except ValueError:
                return [0]

        non_deprecated = [nt for nt in candidates if not nt.deprecated()]
        pool = non_deprecated if non_deprecated else candidates
        return max(pool, key=_version_key)

    def extract_all_categories(self) -> dict[str, dict[str, NodeInfo]]:
        data: dict[str, dict[str, NodeInfo]] = {}
        try:
            for cat_name, cat in sorted(hou.nodeTypeCategories().items()):
                logger.info(f"Processing category: {cat_name}")
                cat_data: dict[str, NodeInfo] = {}
                parent = self.temp_manager.get_parent(cat_name)

                for base_name in sorted(self._get_base_names(cat)):
                    node_type = self._resolve_default_node_type(cat, base_name, parent)
                    if not node_type:
                        continue

                    cat_data[base_name] = self._extract_node_info(node_type, cat_name)

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
