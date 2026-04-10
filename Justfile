PROJECT_ROOT := justfile_directory()
HOUDINI_VEX_PATH := PROJECT_ROOT + "/vex/include"
# Override via HOUDINI_RESOURCES env var for your platform/version
HOUDINI_RESOURCES := env("HOUDINI_RESOURCES", "/Applications/Houdini/Houdini21.0.631/Frameworks/Houdini.framework/Versions/Current/Resources")
# This env var should be set for untrusted localhost.
HOUDINI_RAMEN_TOKEN := env("HOUDINI_RAMEN_TOKEN", "houdini_ramen_secret_2026")
HOUDINI_RAMEN_PORT := env("HOUDINI_RAMEN_PORT", "18080")

# python ==========================================================
fmt-py:
    uv run ruff format tools tests
    uv run python tools/format_domain_graph.py

test-py:
    uv run pytest

# rust ==========================================================
fix-rs:
    cargo clippy --workspace --fix --allow-dirty --allow-staged --all-targets -- -D warnings

fmt-rs:
    just fix-rs
    cargo fmt --all

test-rs:
    cargo test --workspace

# common ========================================================
no-jpn:
    rg '[\p{Han}\p{Hiragana}\p{Katakana}]' src/ crates/ tools/ templates/ tests/ .gitignore Cargo.toml Justfile pyproject.toml README.md

fmt-all: fmt-py fmt-rs

test-all: test-py test-rs

# setup =========================================================
dump-nodes:
    #!/usr/bin/env bash
    set -e

    cd {{ HOUDINI_RESOURCES }}
    source houdini_setup
    cd {{ PROJECT_ROOT }}
    hython tools/dump_nodes.py

generate-api:
    #!/usr/bin/env bash
    set -e

    find "{{ PROJECT_ROOT }}/crates" -mindepth 1 -maxdepth 1 -type d -name "houdini_ramen_*" ! -name "houdini_ramen_core" ! -name "houdini_ramen_helpers" -exec rm -rf {} + || true
    rm -rf "{{ PROJECT_ROOT }}/resources/stubs"

    mkdir -p "{{ PROJECT_ROOT }}/crates"
    mkdir -p "{{ PROJECT_ROOT }}/resources/stubs"

    uv run python tools/generate_rust_api.py \
        "{{ PROJECT_ROOT }}/node_api_dump.json" \
        "{{ PROJECT_ROOT }}/crates" \
        "{{ PROJECT_ROOT }}/resources/stubs"

    uv run python tools/generate_auto_graph.py

    cargo fmt --all

setup-e2e: dump-nodes test-py generate-api test-rs

# context =======================================================
get-context TARGET:
    uv run python tools/compile_context.py {{ TARGET }}

# run ============================================================
houdini-link:
    HOUDINI_VEX_PATH="{{ HOUDINI_VEX_PATH }};&" HOUDINI_RAMEN_TOKEN={{ HOUDINI_RAMEN_TOKEN }} HOUDINI_RAMEN_PORT={{ HOUDINI_RAMEN_PORT }} {{ HOUDINI_RESOURCES }}/bin/houdini tools/link_server.py

run-live TARGET:
    #!/usr/bin/env bash
    set -euo pipefail
    MATCH=$(find examples -maxdepth 1 -type f -name "*{{ TARGET }}*.rs" | sort | head -n 1)
    if [ -z "$MATCH" ]; then
        echo "❌ Error: No example matching '{{ TARGET }}' found."
        exit 1
    fi
    BASENAME=$(basename "$MATCH" .rs)
    echo "🚀 Running: cargo run --example $BASENAME"
    HOUDINI_RAMEN_TOKEN={{ HOUDINI_RAMEN_TOKEN }} HOUDINI_RAMEN_PORT={{ HOUDINI_RAMEN_PORT }} cargo run --example "$BASENAME"

# probe (temp scripts for investigation) ===================================================
[private]
_hython script:
    #!/usr/bin/env bash
    set -e
    cd {{ HOUDINI_RESOURCES }}
    source houdini_setup
    cd {{ PROJECT_ROOT }}
    hython {{ script }}

probe-inner-node: (_hython "tools/probe/probe_inner_node.py")
probe-deep-node: (_hython "tools/probe/probe_deep_node.py")
probe-metadata: (_hython "tools/probe/metadata.py")
explore-api: (_hython "tools/probe/explore_api.py")
explore-label: (_hython "tools/probe/explore_label.py")
ramp-value: (_hython "tools/probe/ramp_interpolation_value.py")

list-types:
    uv run python tools/probe/list_types.py

explore-out-label: (_hython "tools/probe/explore_out_label.py")
explore-out-label2: (_hython "tools/probe/explore_out_label2.py")
analyze-vop-failure: (_hython "tools/probe/analyze_vop_failures.py")
probe_vop_contexts: (_hython "tools/probe/probe_vop_contexts.py")
analyze-failure: (_hython "tools/probe/analyze_other_failures.py")