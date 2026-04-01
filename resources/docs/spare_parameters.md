# Spare Parameters

You can add custom spare parameters to any Houdini node using the `.add_spare()` method combined with parameter builder structs from `crate::core::types`. This is highly readable and scalable.

## Implementation Example
```rust
use houdini_ramen::core::types::{
    SpareFloat, SpareInt, SpareString, SpareToggle, SpareColor, 
    SpareButton, SpareMenu, SpareFile, SpareNodePath, SpareRampFloat, SpareRampColor
};

let ctrl_node = SopNull::new("CONTROLLER")
    .add_spare(SpareFloat::new("radius", "Radius").with_default(2.5).with_range(0.1, 10.0))
    .add_spare(SpareInt::new("count", "Count").with_default(500).with_range(10, 1000))
    .add_spare(SpareString::new("target_name", "Target Name").with_default("geo1"))
    .add_spare(SpareToggle::new("enable_fx", "Enable FX").with_default(true))
    .add_spare(SpareColor::new("bg_color", "Background Color").with_default([0.1, 0.5, 0.9]))
    .add_spare(SpareButton::new("execute", "Execute Script"))
    .add_spare(SpareMenu::new("shape", "Shape Type").with_default(1).add_item("box", "Box").add_item("sphere", "Sphere"))
    .add_spare(SpareFile::new("texture", "Texture").with_default("$HIP/tex.rat"))
    .add_spare(SpareNodePath::new("target_geo", "Target Geo").with_default("../geo1"))
    .add_spare(SpareRampFloat::new("falloff", "Falloff Curve"))
    .add_spare(SpareRampColor::new("color_remap", "Color Remap"));
```
