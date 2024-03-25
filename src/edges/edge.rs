use crate::{loads::{distributed_load::DistributedLoad, load_traits::LoadTraits, point_load::PunctualLoad}, nodes::node::{Node, NodeTraits}};


pub struct Edge {
    start_node: Node,
    end_node: Node,
    distributed_loads: Vec<DistributedLoad>,
    punctual_loads: Vec<PunctualLoad>,
    length: f64,
    ei: f64,
}

pub trait EdgeTraits {
    fn get_moments(&self) -> [f64; 2];
    fn moment_a(&self) -> f64;
    fn moment_b(&self) -> f64;
}

impl Edge {
    pub fn new(
        start_node: Node, 
        end_node: Node, 
        distributed_loads: Vec<DistributedLoad>, 
        punctual_loads: Vec<PunctualLoad>, 
        length: f64, 
        ei: f64
    ) -> Self {
        Edge { start_node, end_node, distributed_loads, punctual_loads, length, ei }
    }
}

impl EdgeTraits for Edge {
    fn get_moments(&self) -> [f64; 2] {
        [self.moment_a(), self.moment_b()]
    }

    fn moment_a(&self) -> f64 {
        2.0 * self.ei * (
            2.0 * self.start_node.get_deflection() + self.end_node.get_deflection() 
            - 3.0 * (
                self.start_node.get_rotation() - self.end_node.get_deflection()
        ) / self.length
    ) / self.length   

    + self.punctual_loads
    .iter()
    .enumerate()
    .fold(
        0.0,
        |accum: f64, 
        (usize , punctual_load) | 
        {
            accum + punctual_load.fem_near(*self)
        }
    )

    + self.distributed_loads
    .iter()
    .enumerate()
    .fold(0.0, |accum: f64, (i, load)| {
        accum + load.fem_near(*self)
    })

    }

    fn moment_b(&self) -> f64 {
        2.0 * self.ei * (
            self.start_node.get_deflection() + 2.0 * self.end_node.get_deflection() 
            - 3.0 * (
                self.start_node.get_rotation() - self.end_node.get_deflection()
        ) / self.length
    ) / self.length

    + self.punctual_loads
    .iter()
    .enumerate()
    .fold(
        0.0,
        |accum: f64, 
        (usize , punctual_load) | 
        {
            accum + punctual_load.fem_far(*self)
        }
    )

    + self.distributed_loads
    .iter()
    .enumerate()
    .fold(0.0, |accum: f64, (i, load)| {
        accum + load.fem_far(*self)
    })
    }
}

// mod edge_tests {
//     use super::Edge;

//     #[test]
//     pub fn test_pointload() {
//     }
// }