use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::core::types::{RampInterpolation, RampPoint, SpareFloat};
use houdini_ramen::generated::sop::{
    SopColor, SopColorColortype, SopNormal, SopPointwrangle, SopSphere, SopSphereType,
};

fn main() {
    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);

    let sphere = graph.add(
        SopSphere::new("sphere")
            .with_type(SopSphereType::PolygonMesh)
            .with_rows(100)
            .with_cols(100),
    );

    let wrangle = graph.add(
        SopPointwrangle::new("noise_deform")
            .set_input(&sphere)
            .with_snippet(include_str!("vex/snippets/003_noise_sphere.vfl"))
            .add_spare(
                SpareFloat::new("freq", "Frequency")
                    .with_default(2.0)
                    .with_range(0.0, 10.0),
            )
            .add_spare(
                SpareFloat::new("amp", "Amplitude")
                    .with_default(0.5)
                    .with_range(0.0, 2.0),
            ),
    );

    let normal = graph.add(
        SopNormal::new("normal")
            .set_input(&wrangle),
    );

    let color = graph.add(
        SopColor::new("color")
            .set_input(&normal)
            .with_colortype(SopColorColortype::RampFromAttribute)
            .with_rampattribute("disp")
            .with_ramprange([-1.0, 1.0])
            .with_ramp(vec![
                RampPoint {
                    position: 0.0,
                    value: vec![0.1, 0.2, 0.8],
                    interpolation: RampInterpolation::Linear,
                },
                RampPoint {
                    position: 0.5,
                    value: vec![0.6, 0.1, 0.8],
                    interpolation: RampInterpolation::Linear,
                },
                RampPoint {
                    position: 1.0,
                    value: vec![1.0, 0.4, 0.1],
                    interpolation: RampInterpolation::Linear,
                },
            ]),
    );

    graph.set_display(&color);

    let python_script = graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}
