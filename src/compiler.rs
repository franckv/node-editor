use std::collections::HashMap;

use egui_snarl::Snarl;

use crate::node::{NodeValueType, NodeView};
use crate::utils::sort;

pub struct GraphCompiler;

impl GraphCompiler {
    pub fn compile<T: NodeView<T> + NodeCompile<T>>(snarl: &Snarl<T>) -> String {
        let mut code = String::from("");

        let mut index_mapping = HashMap::new();

        let nodes = snarl
            .node_ids()
            .enumerate()
            .map(|(idx, (id, node))| {
                index_mapping.insert(id.0, idx);

                node
            })
            .collect::<Vec<&T>>();
        let edges = snarl
            .wires()
            .map(|(out_pin, in_pin)| {
                (
                    index_mapping[&out_pin.node.0],
                    out_pin.output,
                    index_mapping[&in_pin.node.0],
                    in_pin.input,
                )
            })
            .collect::<Vec<(usize, usize, usize, usize)>>();

        tracing::debug!("Nodes {}", nodes.len());
        for edge in &edges {
            tracing::debug!("Edge {}.{}->{}.{}", edge.0, edge.1, edge.2, edge.3);
        }

        let sorted = sort::sort_bfs(&nodes, &edges);

        if sorted.len() != nodes.len() {
            tracing::warn!("Graph contains loops");
        }

        let mut connections = HashMap::new();

        for &(src, src_idx, dst, dst_idx) in &edges {
            connections.insert((dst, dst_idx), (src, src_idx));
        }

        for i in sorted {
            let node = nodes[i];
            tracing::debug!("Compiling {}", node.title());
            let mut inputs = vec![];

            for index in 0..node.inputs().len() {
                if let Some(&(src, src_idx)) = connections.get(&(i, index)) {
                    inputs.push(Some(nodes[src].out_vars(src, src_idx)));
                } else {
                    inputs.push(None);
                }
            }
            code += &node.code(i, &inputs);
            code += "\n";
        }

        code
    }
}

pub struct NodeParam {
    pub name: String,
    pub ty: NodeValueType,
}

pub trait NodeCompile<T> {
    fn out_vars(&self, id: usize, index: usize) -> NodeParam;
    fn code(&self, id: usize, input_vars: &Vec<Option<NodeParam>>) -> String;
}
