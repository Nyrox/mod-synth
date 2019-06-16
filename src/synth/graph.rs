use super::nodes::Node;

pub struct Graph<T> {
    nodes: Vec<GraphNode <T>>,
}

impl<T> Graph<T> {
    pub fn new () -> Self {
        Graph {
            nodes: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn insert(&mut self, node: T) -> usize {
        let node_index = self.nodes.len();

        self.nodes.push(GraphNode { data: node, inputs: Vec::new() });
        
        node_index
    }

    pub fn get_mut (&mut self, index: usize) -> &mut GraphNode <T> {
        &mut self.nodes [index]
    }
}


pub struct GraphNode <T>{
    pub data: T,
    pub inputs: Vec<usize>,
}

impl<T> GraphNode<T> {
    pub fn parent_to (&mut self, node: usize) {
        self.inputs.push (node);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::synth::nodes::{WaveGenerator, WaveType, Sum};

    #[test]
    fn test_basic() {
        let mut graph = Graph::<Box<dyn Node>>::new();

        let end = graph.insert(Box::new(Sum {
            nodes: Vec::new()
        }));
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
        
        graph.get_mut (end).parent_to (in1);
        graph.get_mut (end).parent_to (in2);


        assert!(graph.len() == 3);
    }
}
