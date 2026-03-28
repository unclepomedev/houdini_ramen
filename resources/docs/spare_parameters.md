# Houdini Ramen: Spare Parameters Guidelines

You can add custom spare parameters to any Houdini node using the `add_spare_*` builder methods. This is especially useful for creating control nulls (`SopNull`) to drive other parameters in a network.

Available methods:
- `add_spare_float(name, label, default_value, min, max)`
- `add_spare_int(name, label, default_value, min, max)`
- `add_spare_string(name, label, default_value)`
- `add_spare_toggle(name, label, default_value)`

The `name` should be a valid Houdini internal parameter name (snake_case), and the `label` is the human-readable UI text.

```rust
let ctrl_node = SopNull::new("CONTROLLER")
    .add_spare_float("radius", "Radius", 2.5, 0.1, 10.0)
    .add_spare_int("count", "Count", 500, 10, 1000)
    .add_spare_string("target_path", "Target Path", "../geo1")
    .add_spare_toggle("enable_fx", "Enable FX", true);
```
