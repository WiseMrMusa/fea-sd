use crate::{loads::{distributed_load::DistributedLoad, load_traits::LoadTraits, point_load::PunctualLoad}, nodes::node::{Node, SupportType}};


pub trait SpanBuilder {
    fn new(start_node: Node, end_node: Node, distributed_loads: Vec<DistributedLoad>, point_loads: Vec<PunctualLoad>) -> Self;
}

pub trait SpanDetails {
    fn get_length(&self) -> f64;
    fn get_est_a(&self) -> SupportType;
    fn get_est_b(&self) -> SupportType;
    fn get_est(&self) -> [SupportType; 2] {
        [self.get_est_a(), self.get_est_b()]
    }
}

pub trait SpanFEM {
    fn fem<Load: LoadTraits>(&self, load: Load) -> [f64; 2];
    fn fem_a<Load: LoadTraits>(&self, load: Load) -> f64;
    fn fem_b<Load: LoadTraits>(&self, load: Load) -> f64;
}

pub trait SpanTraits {
    // Member End Moment
    fn mem(&self) -> [f64; 2] {
        [self.mem_a(), self.mem_b()]
    }

    // Member End Moment at support A
    fn mem_a(&self) -> f64;

    // Member End Moment at support B
    fn mem_b(&self) -> f64;


    // Member End Shear
    fn mes(&self) -> [f64; 2] {
        [self.mes_a(), self.mes_b()]
    }

    // Member End Shear at Support A
    fn mes_a(&self) -> f64;
    
    // Member End Shear at Support B
    fn mes_b(&self) -> f64;

    
}