use crate::{
    loads::{distributed_load::DistributedLoad, load_traits::LoadTraits, point_load::PunctualLoad},
    nodes::node::{Node, NodeTraits},
};
use super::traits::{SpanBuilder, SpanDetails, SpanTraits};

#[derive(Debug, Clone)]
pub struct Span {
    start_node: Node,
    end_node: Node,
    length: f64,
    ei: f64,
    distributed_loads: Vec<DistributedLoad>,
    punctual_loads: Vec<PunctualLoad>,
}

impl SpanDetails for Span {
    fn get_length(&self) -> f64 {
        self.length
    }

    fn get_est_a(&self) -> crate::nodes::node::SupportType {
        self.start_node.get_support()
    }

    fn get_est_b(&self) -> crate::nodes::node::SupportType {
        self.end_node.get_support()
    }
}

impl SpanTraits for Span {
    fn mem_a(&self) -> f64 {
        let base_mem = 2.0 * self.ei * (
            2.0 * self.start_node.get_deflection() + self.end_node.get_deflection()
            - 3.0 * (self.start_node.get_rotation() - self.end_node.get_rotation()) / self.length
        ) / self.length;

        let punctual_mem = self.punctual_loads.iter().fold(0.0, |accum, punctual_load| {
            accum + punctual_load.fem_a(self)
        });

        let distributed_mem = self.distributed_loads.iter().fold(0.0, |accum, load| {
            accum + load.fem_a(self)
        });

        base_mem + punctual_mem + distributed_mem
    }

    fn mem_b(&self) -> f64 {
        let base_mem = 2.0 * self.ei * (
            self.start_node.get_deflection() + 2.0 * self.end_node.get_deflection()
            - 3.0 * (self.start_node.get_rotation() - self.end_node.get_rotation()) / self.length
        ) / self.length;

        let punctual_mem = self.punctual_loads.iter().fold(0.0, |accum, punctual_load| {
            accum + punctual_load.fem_b(self)
        });

        let distributed_mem = self.distributed_loads.iter().fold(0.0, |accum, load| {
            accum + load.fem_b(self)
        });

        base_mem + punctual_mem + distributed_mem
    }

    fn mes_a(&self) -> f64 {
        todo!()
    }

    fn mes_b(&self) -> f64 {
        todo!()
    }
}

impl SpanBuilder for Span {
    fn new(start_node: Node, end_node: Node, distributed_loads: Vec<DistributedLoad>, point_loads: Vec<PunctualLoad>) -> Self {
        let length = end_node.get_x() - start_node.get_x();
        Span {
            start_node,
            end_node,
            length,
            distributed_loads,
            punctual_loads: point_loads,
            ei: 1.0,
        }
    }
}

#[cfg(test)]
mod punctual_load_test {
    use crate::nodes::node::SupportType;

    #[test]
    fn test_point_load_with_fixed_ends() {
        use super::*;
        let load = PunctualLoad { value: 40.0, x: 6.0 };

        let start_node = Node::new(0.0, 0.0, SupportType::Fixed);
        let end_node = Node::new(12.0, 0.0, SupportType::Fixed);

        let span = Span::new(start_node, end_node, vec![Default::default()], vec![load, load]);
        assert_eq!(-120.0, span.mem_a());
        assert_eq!(120.0, span.mem_b());
    }

    #[test]
    fn test2_point_load_with_fixed_ends() {
        use super::*;
        let load = PunctualLoad { value: 40.0, x: 2.0 };

        let start_node = Node::new(0.0, 0.0, SupportType::Fixed);
        let end_node = Node::new(4.0, 0.0, SupportType::Fixed);

        let span = Span::new(start_node, end_node, vec![Default::default()], vec![load]);
        assert_eq!(-20.0, span.mem_a());
        assert_eq!(20.0, span.mem_b());
    }
}