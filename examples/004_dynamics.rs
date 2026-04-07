use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::generated::sop::{
    SopSolver, SopSolverInnerExt, SopTestgeometryRubbertoy, SopXform,
};

fn main() {
    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);
    let rubber_toy = graph.add(SopTestgeometryRubbertoy::new("rubber_toy"));
    let solver = graph.add(SopSolver::new("solver1").set_input(&rubber_toy));
    graph.dive_into(&solver, |inner_graph| {
        let prev_frame = inner_graph.prev_frame();
        let out = inner_graph.out();

        let transform1 = inner_graph.add(
            SopXform::new("transform1")
                .set_input(&prev_frame)
                .with_t([-1.0, -2.0, -3.0]),
        );
        inner_graph.connect_existing(&out, 0, &transform1);
    });

    graph.set_display(&solver);
    let python_script = graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}
