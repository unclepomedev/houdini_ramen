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

IGNORED_CATEGORIES = {
    "Cop",
    "Shop",
    "Data",
    "Manager",
    "Director",
    "VopNet",
    "CopNet",
    "ChopNet",
    "TopNet",
}


@dataclass
class ParmInfo:
    name: str
    type: str
    default: Any = None
    menu_items: list[str] = field(default_factory=list)
    menu_labels: list[str] = field(default_factory=list)


@dataclass
class OutputInfo:
    name: str
    label: str
    type: str


@dataclass
class InnerNodeData:
    nodes: dict[str, str] = field(default_factory=dict)
    dive_target: str | None = None


@dataclass
class NodeInfo:
    min_inputs: int
    max_inputs: int
    input_labels: list[str] = field(default_factory=list)
    outputs: list[OutputInfo] = field(default_factory=list)
    parms: list[ParmInfo] = field(default_factory=list)
    builtin_inner_nodes: dict[str, str] = field(default_factory=dict)
    dive_target: str | None = None


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
        self.parents: dict[str, list[hou.Node]] = {}
        self._strategies = {
            "Sop": ("/obj", "geo", "temp_sop_parent"),
            "Chop": ("/ch", "ch", "temp_chop_parent"),
            "Cop2": ("/img", "img", "temp_cop_parent"),
            "Dop": ("/obj", "dopnet", "temp_dop_parent"),
            "Top": ("/tasks", "topnet", "temp_top_parent"),
        }
        self._direct_roots = {
            "Object": "/obj",
            "Lop": "/stage",
            "Driver": "/out",
        }

    def get_parents(self, cat_name: str) -> list[hou.Node]:
        if cat_name in self.parents:
            return self.parents[cat_name]

        parents = []
        try:
            if cat_name == "Vop":
                root = hou.node("/obj")
                if root:
                    geo = root.createNode("geo", "temp_vop_geo", run_init_scripts=False)
                    parents.append(
                        geo.createNode(
                            "attribvop", "temp_attribvop", run_init_scripts=True
                        )
                    )
                mat = hou.node("/mat")
                if mat:
                    parents.append(mat)
            elif cat_name in self._strategies:
                root_path, node_type, node_name = self._strategies[cat_name]
                root = hou.node(root_path)
                if root:
                    parents.append(
                        root.createNode(node_type, node_name, run_init_scripts=False)
                    )
            elif cat_name in self._direct_roots:
                node = hou.node(self._direct_roots[cat_name])
                if node:
                    parents.append(node)
        except Exception as e:
            logger.debug(
                f"Failed to create temp parents for category '{cat_name}': {e}"
            )

        self.parents[cat_name] = parents
        return parents

    def cleanup(self):
        for cat_name, node_list in self.parents.items():
            for node in node_list:
                if not node:
                    continue
                try:
                    # VOP attribvop nodes are created under a temp geo, so destroy the geo parent
                    # Direct roots (/obj, /stage, /out) and /mat should not be destroyed
                    if cat_name == "Vop" and node.type().name() == "attribvop":
                        node.parent().destroy()
                    elif cat_name not in self._direct_roots and not (
                        cat_name == "Vop" and node.path() == "/mat"
                    ):
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
        if not (hasattr(pt, "menuItems") and hasattr(pt, "menuLabels")):
            return info

        items = pt.menuItems()
        labels = pt.menuLabels()
        if items and labels and len(items) == len(labels):
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

    def _create_temp_node(
        self, parents: list[hou.Node], node_type_name: str, run_init_scripts: bool
    ) -> hou.Node | None:
        for parent in parents:
            try:
                return parent.createNode(
                    node_type_name, run_init_scripts=run_init_scripts
                )
            except Exception as e:
                logger.debug(
                    f"Failed to create temp node '{node_type_name}' under '{parent.path()}': {e}"
                )
        return None

    def _destroy_temp_node(self, temp_node: hou.Node | None) -> None:
        if temp_node is not None:
            try:
                temp_node.destroy()
            except Exception as e:
                logger.debug(f"Failed to destroy temp node '{temp_node.name()}': {e}")

    @staticmethod
    def _get_dive_target_path(node: hou.Node) -> str | None:
        hda_def = node.type().definition()
        if not hda_def:
            return None

        sections = hda_def.sections()
        if "DiveTarget" not in sections:
            return None

        dive_path = sections["DiveTarget"].contents().strip()
        return dive_path if dive_path else None

    def _extract_builtin_inner_nodes(
        self, node_type: hou.NodeType, cat_name: str
    ) -> InnerNodeData:
        result = InnerNodeData()
        parents = self.temp_manager.get_parents(cat_name)
        if not parents:
            return result

        temp_node = self._create_temp_node(
            parents, node_type.name(), run_init_scripts=True
        )
        if not temp_node:
            return result

        try:
            target_node = temp_node
            dive_target_path = self._get_dive_target_path(temp_node)
            if dive_target_path:
                dive_node = temp_node.node(dive_target_path)
                if dive_node:
                    result.dive_target = dive_target_path
                    target_node = dive_node
                else:
                    logger.debug(
                        f"Ignoring unresolved DiveTarget for '{node_type.name()}': {dive_target_path}"
                    )

            for child in target_node.children():
                rel_path = child.path().replace(temp_node.path() + "/", "")
                result.nodes[child.name()] = rel_path

        except Exception as e:
            logger.debug(f"Failed to extract inner nodes for {node_type.name()}: {e}")
        finally:
            self._destroy_temp_node(temp_node)

        return result

    def _get_input_labels(
        self, temp_node: hou.Node | None, max_inputs: int
    ) -> list[str]:
        if max_inputs <= 0 or max_inputs >= 128:
            return []

        if temp_node and hasattr(temp_node, "inputLabels"):
            labels = list(temp_node.inputLabels())
            return labels + [""] * (max_inputs - len(labels))

        return [""] * max_inputs

    def _get_outputs(
        self, temp_node: hou.Node | None, max_outputs: int
    ) -> list[OutputInfo]:
        if max_outputs <= 0:
            return []

        outputs = []
        limit = min(max_outputs, 32)

        if temp_node:
            labels = (
                list(temp_node.outputLabels())
                if hasattr(temp_node, "outputLabels")
                else []
            )
            names = (
                list(temp_node.outputNames())
                if hasattr(temp_node, "outputNames")
                else []
            )
            types = (
                list(temp_node.outputDataTypes())
                if hasattr(temp_node, "outputDataTypes")
                else []
            )

            actual_out_count = len(labels)
            loop_count = max(actual_out_count, limit) if actual_out_count > 0 else limit

            for i in range(loop_count):
                label = labels[i] if i < len(labels) else f"Output {i}"
                name = names[i] if i < len(names) else f"output{i}"
                typ = types[i] if i < len(types) else "undef"
                outputs.append(OutputInfo(name=name, label=label, type=typ))

        if not outputs:
            for i in range(limit):
                outputs.append(
                    OutputInfo(name=f"output{i}", label=f"Output {i}", type="undef")
                )

        return outputs

    def _extract_io_info(
        self, node_type: hou.NodeType, cat_name: str
    ) -> tuple[list[str], list[OutputInfo]]:
        max_inputs = node_type.maxNumInputs()
        max_outputs = node_type.maxNumOutputs()

        if max_inputs <= 0 and max_outputs <= 0:
            return [], []

        parents = self.temp_manager.get_parents(cat_name)
        if not parents:
            return self._get_input_labels(None, max_inputs), self._get_outputs(
                None, max_outputs
            )

        temp_node = self._create_temp_node(
            parents, node_type.name(), run_init_scripts=True
        )

        try:
            input_labels = self._get_input_labels(temp_node, max_inputs)
            outputs = self._get_outputs(temp_node, max_outputs)
        except Exception as e:
            logger.debug(
                f"I/O extraction failed for '{cat_name}/{node_type.name()}': {e}"
            )
            input_labels = self._get_input_labels(None, max_inputs)
            outputs = self._get_outputs(None, max_outputs)
        finally:
            self._destroy_temp_node(temp_node)

        return input_labels, outputs

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

        inner_data = self._extract_builtin_inner_nodes(node_type, cat_name)
        input_labels, outputs = self._extract_io_info(node_type, cat_name)

        return NodeInfo(
            min_inputs=node_type.minNumInputs(),
            max_inputs=node_type.maxNumInputs(),
            input_labels=input_labels,
            outputs=outputs,
            parms=parms,
            builtin_inner_nodes=inner_data.nodes,
            dive_target=inner_data.dive_target,
        )

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
    def _resolve_default_node_type(
        cat: hou.NodeTypeCategory, base_key: str, parents: list[hou.Node]
    ):
        if "::" in base_key:
            ns, base = base_key.split("::", 1)
        else:
            ns, base = "", base_key

        create_name = f"{ns}::{base}" if ns else base

        for parent in parents:
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
                return []

        non_deprecated = [nt for nt in candidates if not nt.deprecated()]
        pool = non_deprecated if non_deprecated else candidates
        return max(pool, key=_version_key)

    def extract_all_categories(self) -> dict[str, dict[str, NodeInfo]]:
        data: dict[str, dict[str, NodeInfo]] = {}
        try:
            for cat_name, cat in sorted(hou.nodeTypeCategories().items()):
                if cat_name in IGNORED_CATEGORIES:
                    logger.info(f"Skipping ignored category: {cat_name}")
                    continue

                logger.info(f"Processing category: {cat_name}")
                cat_data: dict[str, NodeInfo] = {}
                parents = self.temp_manager.get_parents(cat_name)

                for base_name in sorted(self._get_base_names(cat)):
                    node_type = self._resolve_default_node_type(cat, base_name, parents)
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
