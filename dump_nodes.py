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


@dataclass
class ParmInfo:
    name: str
    type: str
    default: Any = None


@dataclass
class NodeInfo:
    min_inputs: int
    max_inputs: int
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
        except Exception:
            try:
                tmp_path.unlink(missing_ok=True)
            except OSError:
                logger.warning(
                    f"Failed to clean up temp file: {tmp_path}", exc_info=True
                )
            raise


class HoudiniNodeExtractor:
    """traverses Houdini node information and converts it into a data model"""

    @staticmethod
    def _get_default_value(pt: hou.ParmTemplate) -> Any:
        if not hasattr(pt, "defaultValue"):
            return None
        try:
            return pt.defaultValue()
        except Exception as e:
            logger.debug(f"Could not retrieve default value for '{pt.name()}': {e}")
            return None

    def _extract_single_parm(self, pt: hou.ParmTemplate) -> ParmInfo | None:
        try:
            return ParmInfo(
                name=pt.name(),
                type=pt.type().name(),
                default=self._get_default_value(pt),
            )
        except Exception as e:
            logger.warning(f"Failed to extract parameter info for '{pt.name()}': {e}")
            return None

    def _extract_parms_recursive(self, entries: tuple | list) -> list[ParmInfo]:
        parms = []
        for pt in entries:
            if isinstance(pt, hou.FolderParmTemplate):
                try:
                    parms.extend(self._extract_parms_recursive(pt.parmTemplates()))
                except Exception as e:
                    logger.warning(f"Failed to extract folder '{pt.name()}': {e}")
            else:
                parm_info = self._extract_single_parm(pt)
                if parm_info:
                    parms.append(parm_info)
        return parms

    def _extract_node_info(self, node_type: hou.NodeType) -> NodeInfo:
        parms = []
        try:
            parms = self._extract_parms_recursive(
                node_type.parmTemplateGroup().entries()
            )
        except Exception as e:
            logger.debug(
                f"Could not extract parameters for node '{node_type.name()}': {e}"
            )

        min_inputs, max_inputs = 0, 0
        try:
            min_inputs = node_type.minNumInputs()
            max_inputs = node_type.maxNumInputs()
        except Exception as e:
            logger.warning(
                f"Failed to get input limits for node '{node_type.name()}': {e}"
            )

        return NodeInfo(min_inputs=min_inputs, max_inputs=max_inputs, parms=parms)

    def extract_all_categories(self) -> dict[str, dict[str, NodeInfo]]:
        data: dict[str, dict[str, NodeInfo]] = {}

        for cat_name, cat in sorted(hou.nodeTypeCategories().items()):
            logger.info(f"Processing category: {cat_name}")
            cat_data: dict[str, NodeInfo] = {}

            for node_name, node_type in sorted(cat.nodeTypes().items()):
                cat_data[node_name] = self._extract_node_info(node_type)

            data[cat_name] = cat_data

        return data


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "output",
        nargs="?",
        default="node_api_dump.json",
        help="Output JSON filename (default: node_api_dump.json)",
    )
    args = parser.parse_args()

    output_path = Path.cwd() / args.output

    try:
        logger.info("Starting node data extraction...")

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
