use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::generated::sop::a::SopAttribwrangle;
use houdini_ramen::generated::sop::b::SopBox;
use houdini_ramen::helpers::loops::add_foreach_loop;

fn main() {
    println!("--- Houdini Ramen: Loop Generation Test ---\n");

    let box_node = SopBox::new("base_box").with_size([2.0, 2.0, 2.0]);

    let base_graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create()
        .add_node(&box_node);

    let (graph, loop_end) =
        add_foreach_loop(base_graph, "process_points", &box_node, |g, begin| {
            let wrangle = SopAttribwrangle::new("inner_process")
                .set_input(begin)
                .with_class(1)
                .with_snippet(include_str!("vex/snippets/001_1_yp1.vfl"));

            let g = g.add_node(&wrangle);

            (g, wrangle)
        });

    let final_wrangle = SopAttribwrangle::new("post_process")
        .set_input(&loop_end)
        .with_class(1)
        .with_snippet(include_str!("vex/snippets/001_2_set.vfl"));

    let python_script = graph.add_node(&final_wrangle).build();

    println!("{}", python_script);

    send_to_houdini(&python_script);
}
