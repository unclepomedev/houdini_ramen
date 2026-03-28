# VEX Injection Rules

When using Wrangle nodes (e.g., SopAttribwrangle), you MUST follow these injection rules:

1. Injection Method
   Do NOT inline raw VEX strings in Rust code.
   Always use the `include_str!` macro to read the VEX file.
   Example: `.with_snippet(include_str!("vex/snippets/your_logic.vfl"))`

2. Directory Structure
- `examples/vex/snippets/*.vfl`: Wrangle core logic. Loaded by Rust via `include_str!`.
- `examples/vex/include/*.h`: Header files with pure functions. Included inside `.vfl` files using `#include "name.h"`.
