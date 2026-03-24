import json
from pathlib import Path

import pytest

from generate_rust_api import resolve_rust_type


def test_all_houdini_types_are_supported():
    json_path = Path("node_api_dump.json")
    if not json_path.exists():
        pytest.skip()

    with open(json_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    unique_types = set()
    for cat in data.values():
        for node in cat.values():
            for p in node.get("parms", []):
                if "type" in p:
                    unique_types.add(p["type"])

    unsupported_types = []
    for h_type in unique_types:
        try:
            resolve_rust_type(h_type, None)
        except ValueError:
            unsupported_types.append(h_type)

    assert not unsupported_types
