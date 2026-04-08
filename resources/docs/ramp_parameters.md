# Ramp Parameters

Unlike standard auto-generated node parameters, Ramp parameters (gradients or float curves) rely on hardcoded structs in `core::types`. When a node stub requires a `Vec<RampPoint>`, you MUST construct it using `RampPoint` and `RampInterpolation`.

## Allowed Interpolations

You MUST use one of the following exact variants for `RampInterpolation`:
`Constant`, `Linear`, `CatmullRom`, `MonotoneCubic`, `Bezier`, `BSpline`, `Hermite`.

## Implementation Example

For a Color Ramp (RGB), `value` must contain exactly 3 elements:

```rust
use houdini_ramen::core::types::{RampPoint, RampInterpolation};

let color = graph.add(
    SopColor::new("color")
        .with_ramp(vec![
            RampPoint {
                position: 0.0,
                value: vec![0.1, 0.2, 0.8],
                interpolation: RampInterpolation::Linear,
            },
            RampPoint {
                position: 1.0,
                value: vec![1.0, 0.8, 0.1],
                interpolation: RampInterpolation::CatmullRom,
            },
        ])
);
```

Note: For a Float Ramp, `value` must contain exactly 1 element (e.g., `vec![0.5]`).
