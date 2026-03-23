PROJECT_ROOT := justfile_directory()
# Override via HOUDINI_RESOURCES env var for your platform/version
HOUDINI_RESOURCES := env_var_or_default("HOUDINI_RESOURCES", "/Applications/Houdini/Houdini21.0.631/Frameworks/Houdini.framework/Versions/Current/Resources")

# python ==========================================================
fmt-py:
    uv run ruff format dump_nodes.py
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
    cd -
    hython dump_nodes.py
