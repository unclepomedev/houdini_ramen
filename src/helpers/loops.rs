use crate::core::graph::NodeGraph;
use crate::core::types::HoudiniNode;
use crate::generated::sop::b::{SopBlockBegin, SopBlockEnd};

pub fn add_foreach_loop<F, N>(
    graph: NodeGraph,
    loop_name: &str,
    inner_builder: F,
) -> (NodeGraph, SopBlockEnd)
where
    N: HoudiniNode,
    F: FnOnce(NodeGraph, &SopBlockBegin) -> (NodeGraph, N),
{
    let begin_name = format!("{}_begin", loop_name);
    let end_name = format!("{}_end", loop_name);

    let begin = SopBlockBegin::new(&begin_name);
    let graph = graph.add_node(begin.clone());

    let (graph, last_inner_node) = inner_builder(graph, &begin);

    let end = SopBlockEnd::new(&end_name)
        .set_input(&last_inner_node)
        .with_blockpath(&begin_name);

    let graph = graph.add_node(end.clone());

    (graph, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::sop::b::SopBox;

    #[test]
    fn test_foreach_loop_generation() {
        let base_graph = NodeGraph::new("/obj/geo1");

        let (graph, _loop_end) = add_foreach_loop(base_graph, "test_loop", |g, begin| {
            let inner_node = SopBox::new("inner_box").set_input(begin);
            let g = g.add_node(inner_node.clone());
            (g, inner_node)
        });

        let final_script = graph.build();

        assert!(final_script.contains("createNode('block_begin', 'test_loop_begin')"));
        assert!(final_script.contains("createNode('block_end', 'test_loop_end')"));

        assert!(final_script.contains(".parm('blockpath').set(\"test_loop_begin\")"));

        let end_var_name = final_script
            .lines()
            .find(|line| line.contains("createNode('block_end', 'test_loop_end')"))
            .unwrap()
            .split(" =")
            .next()
            .unwrap()
            .trim();

        let begin_var_name = final_script
            .lines()
            .find(|line| line.contains("createNode('block_begin', 'test_loop_begin')"))
            .unwrap()
            .split(" =")
            .next()
            .unwrap()
            .trim();

        assert!(!begin_var_name.is_empty());
        assert!(!end_var_name.is_empty());
    }
}
