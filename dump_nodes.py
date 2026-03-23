import argparse
import json
import logging
import sys
from pathlib import Path
from typing import Any, Dict, List, Union

import hou

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


def get_default_value(pt: hou.ParmTemplate) -> Any:
    if not hasattr(pt, 'defaultValue'):
        return None
    try:
        return pt.defaultValue()
    except Exception as e:
        logger.debug(f"Could not retrieve default value for parameter '{pt.name()}': {e}")
        return None


def extract_parms(entries: Union[tuple, list]) -> List[Dict[str, Any]]:
    parms = []
    for pt in entries:
        if isinstance(pt, hou.FolderParmTemplate):
            try:
                parms.extend(extract_parms(pt.parmTemplates()))
            except Exception as e:
                logger.warning(f"Failed to extract parameters from folder '{pt.name()}': {e}")
        else:
            try:
                p_info = {
                    "name": pt.name(),
                    "type": pt.type().name()
                }
                default_val = get_default_value(pt)
                if default_val is not None:
                    p_info["default"] = default_val

                parms.append(p_info)
            except Exception as e:
                logger.warning(f"Failed to extract parameter info: {e}")
    return parms


def get_node_data() -> Dict[str, Dict[str, Any]]:
    data: Dict[str, Dict[str, Any]] = {}

    for cat_name, cat in hou.nodeTypeCategories().items():
        logger.info(f"Processing category: {cat_name}")
        cat_data: Dict[str, Any] = {}

        for node_name, node_type in cat.nodeTypes().items():
            parms: List[Dict[str, Any]] = []
            try:
                group = node_type.parmTemplateGroup()
                parms = extract_parms(group.entries())
            except Exception as e:
                logger.debug(f"Could not extract parameters for node '{node_name}' in '{cat_name}': {e}")

            try:
                cat_data[node_name] = {
                    "min_inputs": node_type.minNumInputs(),
                    "max_inputs": node_type.maxNumInputs(),
                    "parms": parms
                }
            except Exception as e:
                logger.warning(f"Failed to get input limits for node '{node_name}': {e}")

        data[cat_name] = cat_data

    return data


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "output",
        nargs="?",
        default="node_api_dump.json",
    )
    args = parser.parse_args()

    output_path = Path.cwd() / args.output

    try:
        logger.info("Starting node data extraction...")
        node_data = get_node_data()

        logger.info(f"Writing data to {output_path}")
        with output_path.open("w", encoding="utf-8") as f:
            json.dump(node_data, f, indent=2)

        logger.info("Extraction completed successfully.")

    except Exception as e:
        logger.error(f"Fatal error during execution: {e}", exc_info=True)
        sys.exit(1)


if __name__ == "__main__":
    main()
