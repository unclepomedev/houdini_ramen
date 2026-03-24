import json
from pathlib import Path

import pytest

from generate_rust_api import resolve_rust_type

DUMP_FILE_NAME = "node_api_dump.json"


def test_all_houdini_types_and_defaults_are_supported():
    json_path = Path(__file__).parent.parent / DUMP_FILE_NAME
    if not json_path.exists():
        pytest.fail(f"{DUMP_FILE_NAME} not found (dump first).")

    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    failed_types = set()

    for cat in data.values():
        for node in cat.values():
            for p in node.get("parms", []):
                h_type = p.get("type")
                if not h_type:
                    continue

                default_val = p.get("default")

                try:
                    resolve_rust_type(h_type, default_val)
                except ValueError:
                    failed_types.add(h_type)

    assert not failed_types, f"Unsupported Houdini types found: {failed_types}"
