PROJECT_ROOT := justfile_directory()
# Override via HOUDINI_RESOURCES env var for your platform/version
HOUDINI_RESOURCES := env_var_or_default("HOUDINI_RESOURCES", "/Applications/Houdini/Houdini21.0.631/Frameworks/Houdini.framework/Versions/Current/Resources")
HOUDINI_RAMEN_TOKEN := env_var_or_default("HOUDINI_RAMEN_TOKEN", "houdini_ramen_secret_2026")
HOUDINI_RAMEN_PORT := env_var_or_default("HOUDINI_RAMEN_PORT", "18080")

# python ==========================================================
fmt-py:
    uv run ruff format tools tests

test-py:
    uv run pytest
# rust ==========================================================
fix-rs:
    cargo clippy --fix --allow-dirty --allow-staged --all-targets -- -D warnings

fmt-rs:
    just fix-rs
    cargo fmt --all

test-rs:
    cargo test

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

    rm -rf "{{ PROJECT_ROOT }}/src/generated"
    rm -rf "{{ PROJECT_ROOT }}/resources/stubs"

    mkdir -p "{{ PROJECT_ROOT }}/src/generated"
    mkdir -p "{{ PROJECT_ROOT }}/resources/stubs"

    uv run python tools/generate_rust_api.py \
        "{{ PROJECT_ROOT }}/node_api_dump.json" \
        "{{ PROJECT_ROOT }}/src/generated" \
        "{{ PROJECT_ROOT }}/resources/stubs"

    cargo fmt

setup-e2e: dump-nodes test-py generate-api test-rs

# debug =======================================================
list-types:
    uv run python tools/list_types.py

explore-api:
    #!/usr/bin/env bash
    set -e

    cd {{ HOUDINI_RESOURCES }}
    source houdini_setup
    cd {{ PROJECT_ROOT }}
    hython tools/explore_api.py

# run ============================================================
# configure according to env
houdini-link:
    HOUDINI_RAMEN_TOKEN={{ HOUDINI_RAMEN_TOKEN }} HOUDINI_RAMEN_PORT={{ HOUDINI_RAMEN_PORT }} {{ HOUDINI_RESOURCES }}/bin/houdini tools/link_server.py

run-live:
    HOUDINI_RAMEN_LIVE_LINK=1 HOUDINI_RAMEN_TOKEN={{ HOUDINI_RAMEN_TOKEN }} HOUDINI_RAMEN_PORT={{ HOUDINI_RAMEN_PORT }} cargo run
