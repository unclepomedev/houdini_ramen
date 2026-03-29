use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::core::types::{RampInterpolation, RampPoint, SpareFloat, SpareInt};
use houdini_ramen::generated::sop::{
    SopColor, SopColorColortype, SopConvertvdb, SopConvertvdbConversion, SopConvertvdbVdbclass,
    SopPointwrangle, SopVdbsmoothsdf, SopVolume, SopVolumewrangle,
};

const RESOLUTION: i32 = 200;
const SCALE: f32 = 0.75;
const SHIFT: f32 = 0.0;
const ITERATIONS: i32 = 5;
const N: f32 = 8.0;

fn main() {
    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);

    let base_volume = graph.add(
        SopVolume::new("base_volume")
            .with_name("density")
            .with_size([3.0, 3.0, 3.0])
            .with_samplediv(RESOLUTION),
    );

    let mandelbulb = graph.add(
        SopVolumewrangle::new("mandelbulb")
            .set_input(&base_volume)
            .with_snippet(include_str!("vex/snippets/002_mandelbulb.vfl"))
            .add_spare(
                SpareFloat::new("scale", "Scale")
                    .with_default(SCALE)
                    .with_range(0.0, 1.0),
            )
            .add_spare(
                SpareFloat::new("shift", "Shift")
                    .with_default(SHIFT)
                    .with_range(0.0, 0.5),
            )
            .add_spare(
                SpareInt::new("iteration", "Iteration")
                    .with_default(ITERATIONS)
                    .with_range(0, 10),
            )
            .add_spare(
                SpareFloat::new("n", "N")
                    .with_default(N)
                    .with_range(0.0, 10.0),
            ),
    );

    let convert_to_sdf = graph.add(
        SopConvertvdb::new("convert_to_sdf")
            .set_input(&mandelbulb)
            .with_conversion(SopConvertvdbConversion::Vdb)
            .with_vdbclass(SopConvertvdbVdbclass::ConvertFogToSdf),
    );

    let smooth = graph.add(
        SopVdbsmoothsdf::new("smooth")
            .set_input(&convert_to_sdf)
            .with_iterations(1),
    );

    let convert_vdb = graph.add(
        SopConvertvdb::new("convert_vdb")
            .set_input(&smooth)
            .with_conversion(SopConvertvdbConversion::Polygons),
    );

    let point_wrangle = graph.add(
        SopPointwrangle::new("point_wrangle")
            .set_input(&convert_vdb)
            .with_snippet(include_str!("vex/snippets/002_point_wrangle.vfl")),
    );
    let color = graph.add(
        SopColor::new("color")
            .set_input(&point_wrangle)
            .with_colortype(SopColorColortype::RampFromAttribute)
            .with_rampattribute("col")
            .with_ramprange([-1.0, 1.0])
            .with_ramp(vec![
                RampPoint {
                    position: 0.0,
                    value: vec![0.1, 0.2, 0.8],
                    interpolation: RampInterpolation::Linear,
                },
                RampPoint {
                    position: 0.5,
                    value: vec![0.8, 0.2, 0.8],
                    interpolation: RampInterpolation::Linear,
                },
                RampPoint {
                    position: 1.0,
                    value: vec![1.0, 0.8, 0.1],
                    interpolation: RampInterpolation::Linear,
                },
            ]),
    );

    graph.set_display(&color);

    let python_script = graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}
