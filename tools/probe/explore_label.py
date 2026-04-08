import logging
import re
from dataclasses import dataclass, field
from typing import Set

import hou

logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")


@dataclass
class MenuAnalysisReport:
    total_menus: int = 0
    empty_labels: Set[str] = field(default_factory=set)
    digit_start_labels: Set[str] = field(default_factory=set)
    special_char_labels: Set[str] = field(default_factory=set)


def categorize_label(
    label: str, node_name: str, parm_name: str, report: MenuAnalysisReport
) -> None:
    if not label or not label.strip():
        report.empty_labels.add(f"Node: {node_name}, Parm: {parm_name}")
        return

    if label[0].isdigit():
        report.digit_start_labels.add(label)

    if not re.match(r"^[a-zA-Z0-9_\s]+$", label):
        report.special_char_labels.add(label)


def analyze_sop_menus() -> MenuAnalysisReport:
    report = MenuAnalysisReport()
    sop_types = hou.sopNodeTypeCategory().nodeTypes().values()

    logging.info(f"Scanning {len(sop_types)} SOP node types safely...")

    for sop_type in sop_types:
        node_name = sop_type.name()

        try:
            ptg = sop_type.parmTemplateGroup()
        except Exception as e:
            logging.debug(f"Skipping node '{node_name}' due to group access error: {e}")
            continue

        for pt in ptg.parmTemplates():
            if not (hasattr(pt, "menuItems") and hasattr(pt, "menuLabels")):
                continue

            items = pt.menuItems()
            labels = pt.menuLabels()

            if not items or not labels or len(items) == 0:
                continue

            report.total_menus += 1

            for label in labels:
                categorize_label(label, node_name, pt.name(), report)

    return report


def print_report_section(title: str, items: Set[str], display_limit: int) -> None:
    print(f"\n{title}: {len(items)}")
    items_list = list(items)
    for item in items_list[:display_limit]:
        print(f"  - '{item}'")
    if len(items_list) > display_limit:
        print("  - ...")


def print_report(report: MenuAnalysisReport) -> None:
    print("\n" + "=" * 50)
    print("ENUM GENERATION ANALYSIS REPORT")
    print("=" * 50)
    print(f"Total Menus Found: {report.total_menus}")

    print_report_section("[Empty Labels Found]", report.empty_labels, 5)
    print_report_section("[Labels Starting with Digits]", report.digit_start_labels, 10)
    print_report_section(
        "[Labels with Special Characters]", report.special_char_labels, 15
    )

    print("=" * 50)


def main() -> None:
    print("=== Safe SOP Menu Analyzer ===")
    report = analyze_sop_menus()
    print_report(report)


if __name__ == "__main__":
    main()
