pub mod core;
pub mod generated;

use crate::core::graph::NodeGraph;
use crate::generated::sop::b::SopBox;
use crate::generated::sop::c::SopColor;
use crate::generated::sop::c::SopCopytopoints;
use crate::generated::sop::m::SopMerge;
use crate::generated::sop::s::SopSphere;

fn main() {
    println!("--- Houdini Ramen: Script Generation Test ---\n");

    let box_node = SopBox::new("base_box").with_size([2.0, 2.0, 2.0]);

    let color_node = SopColor::new("red_color")
        .with_color([1.0, 0.0, 0.0])
        .set_input(&box_node);

    let sphere_node = SopSphere::new("scatter_sphere")
        .with_rad([0.2, 0.2, 0.2])
        .with_type(1)
        .set_input_bounding_source(&box_node);

    let copy_node = SopCopytopoints::new("copy_spheres")
        .set_input_geometry_to_copy(&sphere_node)
        .set_input_target_points_to_copy_to(&color_node);

    let merge_node = SopMerge::new("final_merge")
        .add_input(&box_node)
        .add_input(&copy_node);

    let python_script = NodeGraph::new("/obj/geo1")
        .add_node(box_node)
        .add_node(color_node)
        .add_node(sphere_node)
        .add_node(copy_node)
        .add_node(merge_node)
        .build();

    println!("{}", python_script);
}
