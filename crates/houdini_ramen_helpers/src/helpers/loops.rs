use houdini_ramen_core::graph::NodeGraph;
use houdini_ramen_core::types::HoudiniNode;
use houdini_ramen_sop::{SopBlockBegin, SopBlockEnd};

pub fn add_foreach_loop<F, N, I>(
    graph: &mut NodeGraph,
    loop_name: &str,
    input_node: &I,
    inner_builder: F,
) -> SopBlockEnd
where
    N: HoudiniNode,
    I: HoudiniNode,
    F: FnOnce(&mut NodeGraph, &SopBlockBegin) -> N,
{
    let begin_name = format!("{}_begin", loop_name);
    let end_name = format!("{}_end", loop_name);

    let begin = graph.add(
        SopBlockBegin::new(&begin_name)
            .set_input(input_node)
            .with_blockpath(&format!("../{}", end_name)),
    );

    let last_inner_node = inner_builder(graph, &begin);

    graph.add(
        SopBlockEnd::new(&end_name)
            .set_input(&last_inner_node)
            .with_blockpath(&format!("../{}", begin_name)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use houdini_ramen_sop::SopBox;

    #[test]
    fn test_foreach_loop_generation() {
        let mut graph = NodeGraph::new("/obj/geo1");

        let base_box = graph.add(SopBox::new("base_box"));

        let _loop_end = add_foreach_loop(&mut graph, "test_loop", &base_box, |g, begin| {
            g.add(SopBox::new("inner_box").set_input(begin))
        });

        let final_script = graph.build();

        assert!(final_script.contains("createNode('block_begin', 'test_loop_begin')"));
        assert!(final_script.contains("createNode('block_end', 'test_loop_end')"));
        assert!(final_script.contains(r#".parm('blockpath').set("../test_loop_end")"#));
        assert!(final_script.contains(r#".parm('blockpath').set("../test_loop_begin")"#));
        assert!(final_script.contains("setInput(0, n_base_box_"));
    }
}
