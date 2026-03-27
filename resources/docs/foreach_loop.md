# Houdini Ramen: For-Each Loop Guidelines

When implementing a For-Each loop in Houdini Ramen, you MUST NOT instantiate `SopBlockBegin` or `SopBlockEnd` manually.
Instead, you must use the `add_foreach_loop` helper function from the `helpers` module. This ensures safe scoping, correct input geometry wiring, and automatic boilerplate configuration (e.g., `blockpath`).

## Import Requirement
```rust
use crate::helpers::loops::add_foreach_loop;
```

## Implementation Example

```rust
let box_node = SopBox::new("base_geo");
let base_graph = NodeGraph::new("/obj/geo1").add_node(box_node.clone());

let (graph, loop_end) = add_foreach_loop(base_graph, "my_custom_loop", &box_node,
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
