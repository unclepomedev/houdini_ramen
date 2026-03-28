use crate::core::graph::NodeGraph;
use crate::core::types::HoudiniNode;
use crate::generated::sop::b::{SopBlockBegin, SopBlockEnd};

pub fn add_foreach_loop<F, N, I>(
    graph: NodeGraph,
    loop_name: &str,
    input_node: &I,
    inner_builder: F,
) -> (NodeGraph, SopBlockEnd)
where
    N: HoudiniNode,
    I: HoudiniNode,
    F: FnOnce(NodeGraph, &SopBlockBegin) -> (NodeGraph, N),
{
    let begin_name = format!("{}_begin", loop_name);
    let end_name = format!("{}_end", loop_name);

    let begin = SopBlockBegin::new(&begin_name)
        .set_input(input_node)
        .with_blockpath(&format!("../{}", end_name));

    let graph = graph.add_node(&begin);

    let (graph, last_inner_node) = inner_builder(graph, &begin);

    let end = SopBlockEnd::new(&end_name)
        .set_input(&last_inner_node)
        .with_blockpath(&format!("../{}", begin_name));

    let graph = graph.add_node(&end);

    (graph, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::graph::NodeGraph;
    use crate::generated::sop::b::SopBox;

    #[test]
    fn test_foreach_loop_generation() {
        let base_graph = NodeGraph::new("/obj/geo1");
        let base_box = SopBox::new("base_box");
        let base_graph = base_graph.add_node(&base_box);

        let (graph, _loop_end) =
            add_foreach_loop(base_graph, "test_loop", &base_box, |g, begin| {
                let inner_node = SopBox::new("inner_box").set_input(begin);
                let g = g.add_node(&inner_node);
                (g, inner_node)
            });

        let final_script = graph.build();

        assert!(final_script.contains("createNode('block_begin', 'test_loop_begin')"));
        assert!(final_script.contains("createNode('block_end', 'test_loop_end')"));
        assert!(final_script.contains(r#".parm('blockpath').set("../test_loop_end")"#));
        assert!(final_script.contains(r#".parm('blockpath').set("../test_loop_begin")"#));
        assert!(final_script.contains("setInput(0, n_base_box_"));
    }
}
