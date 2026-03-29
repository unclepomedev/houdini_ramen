# Houdini Ramen: Graph Initialization

When instantiating a `NodeGraph`, you should typically chain `.with_auto_clear()` and `.with_auto_create()` to ensure the target Houdini network exists and is clean before generation.

## Required Imports
```rust
use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::ContainerType::Geo; // Or other ContainerTypes
```

## Implementation Example

```rust
let mut graph = NodeGraph::new("/obj/geo1")
    .with_auto_clear()
    .with_auto_create(Geo);
```
