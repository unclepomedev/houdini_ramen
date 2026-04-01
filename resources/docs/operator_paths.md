# Operator Path

For sibling SOP references (e.g. in `objpath1`), do not use raw node names; prepend `../` (e.g., `../target_node_name`) to avoid `Invalid sop specified` errors.

```rust
let object_merge = SopObjectMerge::new("import_geo")
    .with_objpath1("../target_node_name");
```
