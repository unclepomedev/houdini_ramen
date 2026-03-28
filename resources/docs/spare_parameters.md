# Houdini Ramen: Spare Parameters Guidelines

You can add custom spare parameters to any Houdini node using the `add_spare_*` builder methods. This is especially useful for creating control nulls (`SopNull`) to drive other parameters in a network.

Available methods:
- `add_spare_float(name, label, default_value, min, max)`
- `add_spare_int(name, label, default_value, min, max)`
- `add_spare_string(name, label, default_value)`
- `add_spare_toggle(name, label, default_value)`
- `add_spare_color(name, label, default_value)` - `default_value` expects `[f32; 3]`
- `add_spare_button(name, label)`
- `add_spare_menu(name, label, default_value, menu_items)` - `menu_items` expects `&[(&str, &str)]`
- `add_spare_file(name, label, default_value)`
- `add_spare_node_path(name, label, default_value)`
- `add_spare_ramp_float(name, label)`
- `add_spare_ramp_color(name, label)`

The `name` should be a valid Houdini internal parameter name (snake_case), and the `label` is the human-readable UI text.

```rust
let ctrl_node = SopNull::new("CONTROLLER")
    .add_spare_float("radius", "Radius", 2.5, 0.1, 10.0)
    .add_spare_int("count", "Count", 500, 10, 1000)
    .add_spare_string("target_name", "Target Name", "geo1")
    .add_spare_toggle("enable_fx", "Enable FX", true)
    .add_spare_color("bg_color", "Background Color", [0.1, 0.5, 0.9])
    .add_spare_button("execute", "Execute Script")
    .add_spare_menu("shape", "Shape Type", 1, &[("box", "Box"), ("sphere", "Sphere")])
    .add_spare_file("texture", "Texture", "$HIP/tex.rat")
    .add_spare_node_path("target_geo", "Target Geo", "../geo1")
    .add_spare_ramp_float("falloff", "Falloff Curve")
    .add_spare_ramp_color("color_remap", "Color Remap");
```
