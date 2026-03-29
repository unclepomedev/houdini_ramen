use houdini_ramen::core::graph::NodeGraph;
use houdini_ramen::core::types::ContainerType::Geo;
use houdini_ramen::generated::sop::a::SopAttribwrangle;
use houdini_ramen::generated::sop::b::SopBox;
use houdini_ramen::helpers::loops::add_foreach_loop;

use houdini_ramen::core::types::NODE_ID_COUNTER;
use houdini_ramen::generated::sop::SopAttribwrangleClass;
use std::sync::Mutex;
use std::sync::atomic::Ordering;

static TEST_GLOBAL_LOCK: Mutex<()> = Mutex::new(());

fn reset_node_id() {
    NODE_ID_COUNTER.store(1, Ordering::SeqCst);
}

#[test]
fn test_loop_generation_snapshot() {
    let _lock = TEST_GLOBAL_LOCK.lock().unwrap();
    reset_node_id();

    let mut graph = NodeGraph::new("/obj/geo1")
        .with_auto_clear()
        .with_auto_create(Geo);

    let box_node = graph.add(SopBox::new("base_box").with_size([2.0, 2.0, 2.0]));

    let loop_end = add_foreach_loop(&mut graph, "process_points", &box_node, |g, begin| {
        g.add(
            SopAttribwrangle::new("inner_process")
                .set_input(begin)
                .with_class(SopAttribwrangleClass::Primitives)
                .with_snippet(include_str!("fixtures/vex/001_1_yp1.vfl")),
        )
    });

    let _final_wrangle = graph.add(
        SopAttribwrangle::new("post_process")
            .set_input(&loop_end)
            .with_class(SopAttribwrangleClass::Primitives)
            .with_snippet(include_str!("fixtures/vex/001_2_set.vfl")),
    );

    let python_script = graph.build();
    let expected_script = include_str!("fixtures/expected_loop.py");

    assert_eq!(python_script, expected_script);
}
