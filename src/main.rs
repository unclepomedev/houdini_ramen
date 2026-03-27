pub mod core;
pub mod generated;
pub mod helpers;

use crate::core::graph::NodeGraph;
use crate::core::live_link::send_to_houdini;
use crate::generated::sop::a::SopAttribwrangle;
use crate::generated::sop::b::SopBox;
use crate::helpers::loops::add_foreach_loop;

fn main() {
    println!("--- Houdini Ramen: Loop Generation Test ---\n");

    let box_node = SopBox::new("base_box").with_size([2.0, 2.0, 2.0]);

    let base_graph = NodeGraph::new("/obj/geo1").add_node(box_node.clone());

    let (graph, loop_end) = add_foreach_loop(base_graph, "process_points", |g, begin| {
        let wrangle = SopAttribwrangle::new("inner_process")
            .set_input(begin)
            .with_class(1)
            .with_snippet("@P.y += 1.0;");

        let g = g.add_node(wrangle.clone());

        (g, wrangle)
    });

    let final_wrangle = SopAttribwrangle::new("post_process")
        .set_input(&loop_end)
        .with_class(1)
        .with_snippet("@Cd = set(1, 0, 0);");

    let python_script = graph.add_node(final_wrangle).build();

    println!("{}", python_script);

    send_to_houdini(&python_script);
}
