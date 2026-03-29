# Houdini Ramen: For-Each Loop Guidelines

When implementing a For-Each loop in Houdini Ramen, you MUST NOT instantiate `SopBlockBegin` or `SopBlockEnd` manually.
Instead, you must use the `add_foreach_loop` helper function from the `helpers` module. This ensures safe scoping, correct input geometry wiring, and automatic boilerplate configuration (e.g., `blockpath`).

## Import Requirement
```rust
use crate::helpers::loops::add_foreach_loop;
```

## Implementation Example

```rust
let mut graph = NodeGraph::new("/obj/geo1");
let box_node = graph.add(SopBox::new("base_geo"));

let loop_end = add_foreach_loop(&mut graph, "my_custom_loop", &box_node, |g, begin| {
    g.add(
        SopAttribwrangle::new("inner_process")
            .set_input(begin)
            .with_snippet(include_str!("vex/snippets/your_logic.vfl"))
    )
});

let merge = graph.add(SopMerge::new("final").add_input(&loop_end));
```
