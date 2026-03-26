import json
import logging
import re
from pathlib import Path

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def main():
    project_root = Path(__file__).parent.parent
    stubs_dir = project_root / "resources" / "stubs"

    graph = {}
    struct_re = re.compile(r"pub\s+struct\s+([A-Za-z0-9_]+)\s*;")

    for stub_path in stubs_dir.glob("*.stub"):
        category = stub_path.stem

        with open(stub_path, "r", encoding="utf-8") as f:
            content = f.read()

        for match in struct_re.finditer(content):
            struct_name = match.group(1)
            node_id = f"{category}/{struct_name.lower()}"

            graph[node_id] = {
                "type": "stub",
                "path": str(stub_path.relative_to(project_root)),
                "struct_name": struct_name,
                "depends_on": [],
            }

    output_path = project_root / "resources" / "auto_graph.json"
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(graph, f, indent=2)
    if not graph:
        logger.warning("No structs found in stubs directory. auto_graph.json is empty.")
    else:
        logger.info(f"Generated auto_graph.json with {len(graph)} nodes.")


if __name__ == "__main__":
    main()
