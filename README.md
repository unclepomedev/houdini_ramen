# houdini_ramen

Experimental NaC (Nodes as Code) for Houdini. Declaratively author node networks in Rust, transpile to Python, and execute in Houdini.

## Scope

* **Single Source of Truth:** Version-controlled node networks applying IaC principles.
* **Modularity:** Advanced reusability and abstraction leveraging Rust.
* **AI-Agent Optimization:** Token-efficient generation inspired by MCP and RAG. Maximizes accuracy and minimizes token usage via static typing, compile-time validation, and idiomatic DAGs.

## Out of Scope

* **Drift Detection:** Omitted by design. Prioritizes one-way idempotency (code overwrites DCC state) to maintain a clean baseline while allowing manual GUI debugging.

## Notes

* **Compilation Time:** Initial `cargo build` is heavy due to the massive auto-generated API surface mirroring Houdini's node definitions.
