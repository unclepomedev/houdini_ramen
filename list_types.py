import json
from pathlib import Path

json_path = Path("node_api_dump.json")
if not json_path.exists():
    print("Error: node_api_dump.json not found. Run 'just dump-nodes' first.")
    exit(1)

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
