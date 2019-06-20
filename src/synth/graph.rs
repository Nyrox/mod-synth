use super::nodes::{Node, SamplingContext};

pub struct Graph {
    nodes: Vec<GraphNode>,
    pub out_index: usize,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: Vec::new(), out_index: 0 }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn insert(&mut self, node: Box<Node>) -> usize {
        let node_index = self.nodes.len();

        self.nodes.push(GraphNode {
            data: node,
            inputs: Vec::new(),
        });

        node_index
    }

    pub fn get_mut(&mut self, index: usize) -> &mut GraphNode {
        &mut self.nodes[index]
    }

    pub fn get(&self, index: usize) -> &GraphNode {
        &self.nodes[index]
    }

    pub fn eval_node(&self, ctx: &SamplingContext, index: usize) -> f32 {
        let graph_node = self.get(index);
        let mut inputs = Vec::new();
        for i in graph_node.inputs.iter() {
            inputs.push(self.eval_node(ctx, *i));
        }
        graph_node.data.sample(ctx, inputs)
    }
}

pub struct GraphNode {
    pub data: Box<Node>,
    pub inputs: Vec<usize>,
}

impl GraphNode {
    pub fn parent_to(&mut self, node: usize) {
        self.inputs.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synth::nodes::{Sum, WaveGenerator, WaveType};

    #[test]
    fn test_basic() {
        let mut graph = Graph::new();

        let end = graph.insert(Box::new(Sum {}));
        let in1 = graph.insert(Box::new(WaveGenerator {
            wave_type: WaveType::Sawtooth,
            freq: 440.0,
            offset: 0.0,
        }));
        let in2 = graph.insert(Box::new(WaveGenerator {
            wave_type: WaveType::Square,
            freq: 440.0,
            offset: 0.0,
        }));

        graph.get_mut(end).parent_to(in1);
        graph.get_mut(end).parent_to(in2);

        assert!(graph.len() == 3);
    }
}
