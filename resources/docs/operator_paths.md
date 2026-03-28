# Houdini Ramen: Operator Path Guidelines

When a SOP node requires a reference to another SOP node via an "Operator Path" parameter, you MUST use a relative path.
Directly using the target node's name will result in an `Invalid sop specified` error, as Houdini evaluates paths relative to the current node.

Always prepend `../` to the target node's name to reference a sibling node in the same network level.

```rust
let object_merge = SopObjectMerge::new("import_geo")
    .with_objpath1("../target_node_name");
```
