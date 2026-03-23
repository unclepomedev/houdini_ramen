import argparse
import json
import logging
import os
import sys
import tempfile
from pathlib import Path

import hou

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


class HoudiniJSONEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, tuple):
            return list(obj)
        return super().default(obj)


def get_default_value(pt: hou.ParmTemplate):
    if not hasattr(pt, "defaultValue"):
        return None
    try:
        return pt.defaultValue()
    except Exception as e:
        logger.debug(
            f"Could not retrieve default value for parameter '{pt.name()}': {e}"
        )
        return None


def extract_parms(entries: tuple | list) -> list[dict[str, object]]:
    parms = []
    for pt in entries:
        if isinstance(pt, hou.FolderParmTemplate):
            try:
                parms.extend(extract_parms(pt.parmTemplates()))
            except Exception as e:
                logger.warning(
                    f"Failed to extract parameters from folder '{pt.name()}': {e}"
                )
        else:
            try:
                p_info = {"name": pt.name(), "type": pt.type().name()}
                default_val = get_default_value(pt)
                if default_val is not None:
                    p_info["default"] = default_val

                parms.append(p_info)
            except Exception as e:
                logger.warning(f"Failed to extract parameter info: {e}")
    return parms


def get_node_data() -> dict[str, dict[str, object]]:
    data: dict[str, dict[str, object]] = {}

    for cat_name, cat in sorted(hou.nodeTypeCategories().items()):
        logger.info(f"Processing category: {cat_name}")
        cat_data: dict[str, object] = {}

        for node_name, node_type in sorted(cat.nodeTypes().items()):
            parms: list[dict[str, object]] = []
            try:
                group = node_type.parmTemplateGroup()
                parms = extract_parms(group.entries())
            except Exception as e:
                logger.debug(
                    f"Could not extract parameters for node '{node_name}' in '{cat_name}': {e}"
                )

            node_entry: dict[str, object] = {"parms": parms}
            try:
                node_entry["min_inputs"] = node_type.minNumInputs()
                node_entry["max_inputs"] = node_type.maxNumInputs()
            except Exception as e:
                logger.warning(
                    f"Failed to get input limits for node '{node_name}': {e}"
                )
            cat_data[node_name] = node_entry
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
        node_data = get_node_data()

        logger.info(f"Writing data to {output_path}")
        output_path.parent.mkdir(parents=True, exist_ok=True)
        fd, tmp_name = tempfile.mkstemp(
            prefix=f".{output_path.name}.",
            suffix=".tmp",
            dir=output_path.parent,
        )
        tmp_path = Path(tmp_name)
        try:
            with os.fdopen(fd, "w", encoding="utf-8") as f:
                json.dump(node_data, f, indent=2, cls=HoudiniJSONEncoder)
                f.write("\n")
                f.flush()
                os.fsync(f.fileno())
            os.replace(tmp_path, output_path)
        except Exception:
            try:
                tmp_path.unlink(missing_ok=True)
            except OSError:
                logger.warning(
                    f"Failed to clean up temp file: {tmp_path}", exc_info=True
                )
            raise

        logger.info("Extraction completed successfully.")

    except Exception as e:
        logger.error(f"Fatal error during execution: {e}", exc_info=True)
        sys.exit(1)


if __name__ == "__main__":
    main()
