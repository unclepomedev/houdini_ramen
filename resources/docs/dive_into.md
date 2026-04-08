# Subnets & `dive_into`

Use `graph.dive_into` to construct graphs inside container nodes like `solver` or `vellumsolver`.

## Rules

1. **Import Trait:** ALWAYS import the specific container's extension trait (e.g., `SopSolverInnerExt`) to access auto-generated methods for pre-existing internal nodes.
2. **Fetch Existing Nodes:** NEVER instantiate built-in nodes (e.g., `Prev_Frame`, `OUT`) manually. Fetch them using `inner_graph.<node_name>()`.
3. **Wiring Existing -> New:** Pass the existing node reference to `.set_input()`.
4. **Wiring New -> Existing:** Use `inner_graph.connect_existing(&dst, input_idx, &src)`. NEVER use raw string paths for wiring.

## Implementation Example

```rust
use houdini_ramen::sop::{SopSolver, SopSolverInnerExt, SopXform};

let solver = graph.add(SopSolver::new("solver1").set_input(&base_geo));

graph.dive_into(&solver, |inner_graph| {
    let prev_frame = inner_graph.prev_frame();
    let out = inner_graph.out();

    let transform = inner_graph.add(
        SopXform::new("transform1")
            .set_input(&prev_frame)
            .with_t([-1.0, -2.0, -3.0]),
    );

    inner_graph.connect_existing(&out, 0, &transform);
});
```
