import json
from collections import Counter
from pathlib import Path


def analyze_other_failures():
    report_path = Path("probe_io_report.json")
    if not report_path.exists():
        print(f"File not found: {report_path}")
        return

    with open(report_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    failures = data.get("failures_instantiation", {})

    for cat in sorted(failures.keys()):
        cat_failures = failures[cat]
        if not cat_failures:
            continue

        print(f"\n=== {cat} Instantiation Failures: {len(cat_failures)} total ===")
        reasons = Counter([item.get("reason", "Unknown") for item in cat_failures])
        for reason, count in reasons.most_common():
            print(f"{count:>5} times: {reason}")

        samples = [item.get("node") for item in cat_failures]
        print(f"Samples: {', '.join(samples[:15])}{'...' if len(samples) > 15 else ''}")


if __name__ == "__main__":
    analyze_other_failures()
