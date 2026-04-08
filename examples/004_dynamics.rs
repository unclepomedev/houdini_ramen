use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen_sop::{
    SopAttribnoise, SopAttribnoiseAttribtype, SopAttribnoiseBasis, SopAttribnoiseOperation,
    SopAttribvop, SopAttribvopBindclass, SopAttribvopInnerExt, SopScatter, SopSolver,
    SopSolverInnerExt, SopTestgeometryRubbertoy,
};
use houdini_ramen_vop::VopAdd;

fn main() {
    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);
    let rubber_toy = graph.add(SopTestgeometryRubbertoy::new("rubber_toy"));
    let solver = graph.add(SopSolver::new("solver1").set_input(&rubber_toy));
    graph.dive_into(&solver, |inner_graph| {
        let prev_frame = inner_graph.prev_frame();
        let out = inner_graph.out();
        let scatter = inner_graph.add(SopScatter::new("scatter1").set_input(&prev_frame));
        let noise = inner_graph.add(
            SopAttribnoise::new("noise1")
                .set_input(&scatter)
                .with_attribtype(SopAttribnoiseAttribtype::Vector)
                .with_attribs("v")
                .with_operation(SopAttribnoiseOperation::Set)
                .with_basis(SopAttribnoiseBasis::PerlinFlow),
        );
        let pointvop1 = inner_graph.add(
            SopAttribvop::new("pointvop1")
                .set_input(&noise)
                .with_bindclass(SopAttribvopBindclass::Points),
        );
        inner_graph.dive_into(&pointvop1, |vop_graph| {
            let in1 = vop_graph.geometryvopglobal1();
            let out1 = vop_graph.geometryvopoutput1();
            let add1 = vop_graph.add(VopAdd::new("add1").set_input(&in1));
            vop_graph.connect_existing(&out1, 0, &add1);
        });

        inner_graph.connect_existing(&out, 0, &pointvop1);
    });

    graph.set_display(&solver);
    let python_script = graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}
