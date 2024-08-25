use std::collections::HashMap;

use egui_snarl::Snarl;

use crate::node::NodeView;
use crate::utils::sort;

pub struct GraphCompiler {}

impl GraphCompiler {
    pub fn compile<T: NodeView<T>>(&self, snarl: &Snarl<T>) {
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
                    index_mapping[&in_pin.node.0],
                )
            })
            .collect::<Vec<(usize, usize)>>();

        tracing::debug!("Nodes {}", nodes.len());
        for edge in &edges {
            tracing::debug!("Edge {}->{}", edge.0, edge.1);
        }

        let sorted = sort::sort_bfs(&nodes, edges);

        if sorted.len() != nodes.len() {
            tracing::warn!("Graph contains loops");
        }

        for i in sorted {
            tracing::info!("{}", nodes[i].title());
        }
    }
}
