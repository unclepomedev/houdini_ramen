# Houdini Ramen: SopSphere Parameter Dependencies

The `SopSphere` node has implicit parameter dependencies based on its `type` (Primitive Type). If you want to increase the resolution of the sphere, you MUST set the parameters according to the selected type:

- For `SopSphereType::PolygonMesh`: You MUST use `.with_rows()` and `.with_cols()` to set the resolution.
- For `SopSphereType::Polygon` (Triangles): `rows` and `cols` are ignored. You MUST use `.with_freq()` instead.

If you are asked to create a high-resolution sphere for deformation, `PolygonMesh` with `rows` and `cols` is generally preferred.
