# houdini_ramen

Experimental NaC (Nodes as Code) for Houdini. Declaratively author node networks in Rust, transpile to Python, and execute in Houdini.

## Scope

* **Single Source of Truth:** Version-controlled node networks applying IaC principles.
* **Modularity:** Advanced reusability and abstraction leveraging Rust.

## Out of Scope

* **Drift Detection:** Omitted by design. Prioritizes one-way idempotency (code overwrites DCC state) to maintain a clean baseline while allowing manual GUI debugging.
* **AI-Agent Optimization:**
    * Out of scope for now. Long-term goal: token-efficient generation via static typing, compile-time validation, and idiomatic DAGs — inspired by MCP and RAG patterns.
      Image input has been identified as a key requirement; agentic workflow validation is deferred until that foundation is in place.

## Notes

* **Compilation Time:** Initial `cargo build` is heavy due to the massive auto-generated API surface mirroring Houdini's node definitions.
