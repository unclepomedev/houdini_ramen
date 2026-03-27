# Houdini Ramen Agent Guidelines

## 1. Core Philosophy & Identity
Your primary directive is **Deterministic Execution**. You are strictly prohibited from hallucinating or guessing Houdini Node APIs, parameters, or VEX injection strategies. You must rely 100% on the DAG-based Context Compiler.

## 2. Context Compilation (The DAG System)
Before writing or modifying any Rust code related to Houdini nodes, you MUST fetch the exact stubs and domain guidelines. 

**How to find a `<target_id>`:**
- **For Houdini Nodes:** The ID format is strictly `{category}/{struct_name}` in lowercase.
- **Target Examples:**
  - Specific Node: `sop/sopattribwrangle`, `sop/sopbox`, `dop/dopnetwork`
  - Task/Recipe: `task/foreach_loop`, `task/your_custom_recipe_name`
- **If unsure:** Do not guess. Search the auto-generated registry directly using standard terminal tools:
  ```bash
  grep -i "target_node_name" resources/auto_graph.json
  ```

**Command:**
```bash
just get-context <target_id>
```

**Execution Rule:**
Read the standard output of the command. **If the output contains `[ERROR]` markers, STOP and report via the Observation Loop.** Otherwise, it will provide the exact Rust `pub struct` definitions and associated Markdown rules (e.g., VEX injection rules). Build your code *strictly* based on this output. Do not assume standard Houdini parameters exist unless they are present in the compiled stub.

## 3. Verification Step (LiveLink)
Once you have implemented the Rust script inside the `examples/` directory based on the compiled context, you MUST verify your implementation by executing it. Assume the Admin has already started the Houdini LiveLink server.

**Command:**
```bash
just run-live <target_prefix_or_name>
```
*Example: If you created `examples/02_scatter.rs`, run `just run-live 02`.*

**Evaluation Rule:**
- **Success:** If the execution completes without Rust compilation errors or Houdini Python runtime errors, your task is successful.
- **Failure:** If you encounter a Rust compiler error or a runtime error returned from Houdini, proceed immediately to the Observation Loop.

## 4. The Observation Loop (Feedback & Knowledge Growth)
The Context Compiler's knowledge base is continually evolving. If you encounter any of the following situations during verification:
- A Rust compilation error due to a missing method in a stub.
- A runtime error from Houdini via LiveLink.
- A semantic gap where you don't know the best practice for a specific Houdini task.

**DO NOT brute-force, guess, or repeatedly retry.** Instead, halt your implementation and report the deficiency to the Admin (Human) using the following exact format in your chat response:

```text
[OBSERVATION]
- Event Type: [compile_miss | livelink_error | missing_guideline]
- Target Node/Task: [e.g., sop/sopboolean]
- Detail: [e.g., The stub is missing the method to set the output group name.]
- Proposal: [e.g., Please update the API dumper or add a specific rule to domain_graph.json.]
```
Wait for the Admin to update the graph and stubs before proceeding.

## 5. Strict Coding Standards
- **Code Comments:** NEVER include Japanese (or any non-English) comments or obvious, self-explanatory comments inside code blocks.
- **Code Explanations:** Explanations of the code must be written in the normal chat text outside the code blocks, never inside.
- **VEX Injection:** Never inline raw VEX strings in Rust. Always follow the `include_str!` rules fetched via the context compiler.
