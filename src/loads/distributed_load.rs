use crate::spans::span::Span;

use super::load_traits::LoadTraits;

#[derive(Debug, Default, Clone, Copy)]
pub struct DistributedLoad {
    pub start_value : f64,
    pub end_value : f64,
    pub x0: f64,
    pub xf: f64,
    pub tg: f64,
    pub x_centroid: f64,
    pub total_force: f64,
}

// TODO: Not yet implemented
impl LoadTraits for DistributedLoad {
    
    fn fem_a(&self, span: &Span) -> f64 {
        0.0
    }
    
    fn fem_b(&self, span: &Span) -> f64 {
        0.0
    }
}