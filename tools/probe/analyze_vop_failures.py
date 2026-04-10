import json
from collections import Counter
from pathlib import Path


def analyze_failures():
    report_path = Path("probe_io_report.json")
    if not report_path.exists():
        print(f"File not found: {report_path}")
        return

    with open(report_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    vop_failures = data.get("failures_instantiation", {}).get("Vop", [])

    if not vop_failures:
        print("No VOP failures found in the report.")
        return

    reasons = Counter([item.get("reason", "Unknown") for item in vop_failures])

    print(f"=== VOP Instantiation Failures: {len(vop_failures)} total ===")
    for reason, count in reasons.most_common():
        print(f"{count:>5} times: {reason}")

    print("\n=== Sample nodes for 'Invalid node type name' ===")
    samples = [
        item.get("node")
        for item in vop_failures
        if "Invalid node type name" in item.get("reason", "")
    ]
    print(", ".join(samples[:20]) + ("..." if len(samples) > 20 else ""))


if __name__ == "__main__":
    analyze_failures()
