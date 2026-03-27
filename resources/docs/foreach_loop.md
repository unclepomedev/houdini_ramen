# Houdini Ramen: For-Each Loop Guidelines

When implementing a For-Each loop in Houdini Ramen, you MUST NOT instantiate `SopBlockBegin` or `SopBlockEnd` manually.
Instead, you must use the `add_foreach_loop` helper function from the `helpers` module. This ensures safe scoping and automatic boilerplate configuration (e.g., `blockpath`).

### Import Requirement
```rust
use crate::helpers::loops::add_foreach_loop;
```

### Implementation Example

```rust
let (graph, loop_end) = add_foreach_loop(
    base_graph,
    "my_custom_loop",
    |g, begin| {
        let wrangle = SopAttribwrangle::new("inner_process")
            .set_input(begin)
            .with_snippet("@P.y += 1.0;");

        let g = g.add_node(wrangle.clone());
        
        (g, wrangle)
    },
);

let merge = SopMerge::new("final").add_input(&loop_end);
let graph = graph.add_node(merge);
```
