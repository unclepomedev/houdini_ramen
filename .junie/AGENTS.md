# Houdini Ramen Agent Guidelines

## 1. Core Philosophy

**Deterministic Execution ONLY**. NEVER hallucinate or guess Houdini APIs, parameters, or VEX. Rely 100% on the DAG Context Compiler.

## 2. Context Compilation

ALWAYS fetch stubs/guidelines before coding.

- **Target ID Format:** Lowercase `{category}/{struct_name}` (e.g., `sop/sopbox`, `task/foreach_loop`).
- **Find IDs:** NEVER guess. Use terminal:
  ```bash
  grep -i "node_name" resources/auto_graph.json
  grep -E "^  \"(task|doc)/" resources/domain_graph.json
  ```

- **Command:** `just get-context <target_id>`
- **Rule:** Build strictly from stdout. Do NOT assume unlisted parameters exist. If stdout contains `[ERROR]`, STOP and
  use Observation Loop.

## 3. Verification (LiveLink)

Verify `examples/` scripts (assuming LiveLink server is running).

- **Command:** `just run-live <prefix_or_name>` (e.g., `just run-live 002`).
- **Rule:** Success = No Rust/Houdini errors. Failure = STOP and use Observation Loop immediately.

## 4. Observation Loop

If you encounter missing stub methods, LiveLink errors, semantic gaps, or stale docs: **NEVER brute-force, guess, or retry**. Halt and report to the Admin EXACTLY like this:

```text
[OBSERVATION]
- Event Type: [compile_miss | livelink_error | missing_guideline | stale_doc]
- Target Node/Task: [e.g., sop/sopboolean]
- Detail: [Specific error or missing method]
- Proposal: [Suggested fix for Admin]
```

Wait for Admin updates before proceeding.

## 5. Coding Standards

- **Comments:** NO Japanese/non-English. NO obvious/self-explanatory comments inside code.
- **Explanations:** Explain outside code blocks ONLY.
- **VEX:** NO inline raw strings. ALWAYS use `include_str!` rules from context.
- **Node Passing:** ALWAYS pass by reference to `set_input`/`add_node` (e.g., `&my_node`). NEVER use `.clone()`.
