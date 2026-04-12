use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::live_link::send_to_houdini;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::sop::{
    SopAttribnoise, SopAttribnoiseAttribtype, SopAttribnoiseBasis, SopAttribnoiseOperation,
    SopAttribvop, SopAttribvopBindclass, SopScatter, SopTestgeometryRubbertoy,
};
use houdini_ramen_core::graph::InnerGraph;
use houdini_ramen_core::types::NodeOutput;
use houdini_ramen_sop::{SopAttribvopInnerExt, SopSolver, SopSolverInnerExt};
use houdini_ramen_vop::{
    VopAdd, VopConstant, VopGeometryvopglobal, VopGeometryvopglobalOutputs, VopGeometryvopoutput,
    VopGeometryvopoutputWiringExt, VopMultiply,
};

const TIMESTEP: f32 = 1.0_f32 / 24.0_f32;

fn main() {
    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);

    let init_noise = build_initial_state(&mut graph);
    let solver1 = build_dynamics_solver(&mut graph, &init_noise);
    graph.set_display(&solver1);

    let python_script = graph.build();
    println!("{}", python_script);
    send_to_houdini(&python_script);
}

/// Constructing the initial state (from generating the rubber toy to assigning initial velocity)
fn build_initial_state(graph: &mut NodeGraph) -> NodeOutput {
    let rubber_toy = graph.add(SopTestgeometryRubbertoy::new("rubber_toy"));
    let scatter1 = graph.add(SopScatter::new("scatter1").set_input(&rubber_toy));

    let init_noise = graph.add(
        SopAttribnoise::new("init_noise")
            .set_input(&scatter1)
            .with_attribtype(SopAttribnoiseAttribtype::Vector)
            .with_attribs("v")
            .with_operation(SopAttribnoiseOperation::Set)
            .with_basis(SopAttribnoiseBasis::PerlinFlow),
    );
    NodeOutput::from(&init_noise)
}

/// Build a simulation loop inside the solver.
fn build_dynamics_solver(graph: &mut NodeGraph, input_node: impl Into<NodeOutput>) -> NodeOutput {
    let solver1 = graph.add(SopSolver::new("solver1").set_input(input_node));

    graph.dive_into(&solver1, |inner_graph| {
        let prev_frame = inner_graph.prev_frame();
        let out = inner_graph.out();

        let noise1 = inner_graph.add(
            SopAttribnoise::new("noise1")
                .set_input(&prev_frame)
                .with_attribtype(SopAttribnoiseAttribtype::Vector)
                .with_attribs("v")
                .with_operation(SopAttribnoiseOperation::Add)
                .with_basis(SopAttribnoiseBasis::PerlinFlow)
                .with_animated(true),
        );

        let pointvop1 = inner_graph.add(
            SopAttribvop::new("pointvop1")
                .set_input(&noise1)
                .with_bindclass(SopAttribvopBindclass::Points),
        );

        inner_graph.dive_into(&pointvop1, build_velocity_update_vop);
        inner_graph.connect_existing(&out, 0, &pointvop1);
    });

    NodeOutput::from(&solver1)
}

/// Build the position update logic inside VOP.
fn build_velocity_update_vop(vop_graph: &mut InnerGraph<'_, SopAttribvop>) {
    let in1 = vop_graph
        .geometryvopglobal1()
        .typed_as::<VopGeometryvopglobal>();
    let out1 = vop_graph
        .geometryvopoutput1()
        .typed_as::<VopGeometryvopoutput>();

    let const1 = vop_graph.add(VopConstant::new("timestep").with_floatdef(TIMESTEP));

    let multiply1 = vop_graph.add(
        VopMultiply::new("multiply1")
            .set_input(in1.out_v())
            .set_input_at(1, &const1),
    );

    let add1 = vop_graph.add(
        VopAdd::new("add1")
            .set_input(in1.out_p())
            .set_input_at(1, &multiply1),
    );

    vop_graph.wire(&out1).set_input_name_p(&add1);
}
