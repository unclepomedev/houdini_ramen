import json

with open("node_api_dump.json", "r", encoding="utf-8") as f:
    data = json.load(f)

unique_types = set()
for cat in data.values():
    for node in cat.values():
        for p in node.get("parms", []):
            if "type" in p:
                unique_types.add(p["type"])

for t in sorted(unique_types):
    print(t)
