# Operator Path

NEVER use raw node names. ALWAYS prepend `../` for sibling SOP references to prevent `Invalid sop specified` errors.

```rust
let object_merge = SopObjectMerge::new("import_geo")
    .with_objpath1("../target_node_name");
```
