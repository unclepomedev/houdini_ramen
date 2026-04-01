# Display & Render Flags

To automatically show the final result in the Houdini viewport, you MUST specify the output node using the `graph.set_display()` method before building the graph. Do not attempt to set display flags via node parameters or custom Python/VEX snippets.

## Implementation Example

```rust
let mut graph = NodeGraph::new("/obj/geo1");

let box_node = graph.add(SopBox::new("base_geo"));
let color_node = graph.add(
    SopColor::new("color")
        .set_input(&box_node)
        .with_color([1.0, 0.0, 0.0])
);

// Explicitly set the display and render flags to the final output node
graph.set_display(&color_node);

let python_script = graph.build();
```
