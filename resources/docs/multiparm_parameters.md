# Multiparm (Instanced) Parameters

When a Houdini node has multiparm blocks (e.g., `Attribute Create` or `Merge`), the parameters are generated with an `_inst` suffix and require a 1-based index as the first argument.

- You MUST use the `_inst` method when it is present in the stub.
- The index is 1-based. To configure the first item, pass `1`, not `0`.

## Implementation Example
```rust
let life = graph.add(
    SopAttribcreate::new("life")
        .set_input(&scatter)
        .with_type_inst(1, SopAttribcreateType::Integer)
        .with_name_inst(1, "life")
);
```
