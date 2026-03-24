import json
import sys
from pathlib import Path

DUMP_FILE_NAME = "node_api_dump.json"


def main():
    json_path = Path(DUMP_FILE_NAME)
    if not json_path.exists():
        print(f"Error: {DUMP_FILE_NAME} not found. Dump first.")
        sys.exit(1)

    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    unique_types = set()
    for cat in data.values():
        for node in cat.values():
            for p in node.get("parms", []):
                if "type" in p:
                    unique_types.add(p["type"])

    for t in sorted(unique_types):
        print(t)


if __name__ == "__main__":
    main()
