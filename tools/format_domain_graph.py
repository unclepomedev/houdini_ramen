import sys
import json
from pathlib import Path


def main():
    root_dir = Path(__file__).parent.parent
    target_file = root_dir / "resources" / "domain_graph.json"

    if not target_file.exists():
        print(f"❌ Error: {target_file.name} not found.")
        sys.exit(1)

    try:
        with open(target_file, "r", encoding="utf-8") as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        print(f"❌ Error: Invalid JSON in {target_file.name}: {e}")
        sys.exit(1)

    with open(target_file, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2, ensure_ascii=False, sort_keys=True)
        f.write("\n")

    print(f"✅ Sorted and formatted: {target_file.name}")


if __name__ == "__main__":
    main()
