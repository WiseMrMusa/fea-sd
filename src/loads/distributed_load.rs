use crate::{
    nodes::node::SupportType,
    spans::{span::Span, traits::SpanDetails},
};

use super::load_traits::LoadTraits;

#[derive(Debug, Default, Clone, Copy)]
pub struct DistributedLoad {
    pub start_value: f64,
    pub end_value: f64,
    pub x0: f64,
    pub xf: f64,
    pub tg: f64,
    pub x_centroid: f64,
    pub total_force: f64,
}

impl LoadTraits for DistributedLoad {
    fn fem_a(&self, span: &Span) -> f64 {
        let l = span.get_length();
        let w = self.start_value - self.end_value;

        match (span.get_est_a(), span.get_est_b()) {
            (SupportType::Fixed, SupportType::Fixed) => {
                if self.start_value == self.end_value {
                    - (self.start_value * l * l) / 12.0
                } else {
                    - (w * l * l) / 20.0 + (self.end_value * l * l) / 12.0
                }
            }
            (SupportType::Fixed, SupportType::Roller)
            | (SupportType::Fixed, SupportType::Hinged)
            | (SupportType::Roller, SupportType::Fixed)
            | (SupportType::Roller, SupportType::Roller)
            | (SupportType::Roller, SupportType::Hinged)
            | (SupportType::Hinged, SupportType::Fixed)
            | (SupportType::Hinged, SupportType::Roller)
            | (SupportType::Hinged, SupportType::Hinged) => {
                if self.start_value == self.end_value {
                    - (self.start_value * l * l) / 8.0
                } else {
                    - (w * l * l) / 20.0 + (self.end_value * l * l) / 15.0
                }
            }
        }
    }

    fn fem_b(&self, span: &Span) -> f64 {
        let l = span.get_length();
        let w = self.start_value - self.end_value;

        match (span.get_est_a(), span.get_est_b()) {
            (SupportType::Fixed, SupportType::Fixed) => {
                if self.start_value == self.end_value {
                    (self.start_value * l * l) / 12.0
                } else {
                    (w * l * l) / 30.0 + (self.end_value * l * l) / 12.0
                }
            }
            (SupportType::Fixed, SupportType::Roller)
            | (SupportType::Fixed, SupportType::Hinged)
            | (SupportType::Roller, SupportType::Fixed)
            | (SupportType::Roller, SupportType::Roller)
            | (SupportType::Roller, SupportType::Hinged)
            | (SupportType::Hinged, SupportType::Fixed)
            | (SupportType::Hinged, SupportType::Roller)
            | (SupportType::Hinged, SupportType::Hinged) => 0.0,
        }
    }
}

#[cfg(test)]
mod distributed_load_tests {
    use super::*;
    use crate::{nodes::node::{Node, NodeTraits}, spans::traits::SpanBuilder};

    #[test]
    fn test_distributed_load_with_fixed_supports() {
        let start_node = Node::new(0.0, 0.0, SupportType::Fixed);
        let end_node = Node::new(6.0, 0.0, SupportType::Fixed);
        let span = Span::new(start_node, end_node, vec![DistributedLoad {
            start_value: 20.0,
            end_value: 20.0,
            x0: 0.0,
            xf: 6.0,
            tg: 0.0,
            x_centroid: 3.0,
            total_force: 120.0,
        }], vec![]);

        let distributed_load = DistributedLoad {
            start_value: 20.0,
            end_value: 20.0,
            x0: 0.0,
            xf: 6.0,
            tg: 0.0,
            x_centroid: 3.0,
            total_force: 120.0,
        };
        assert_eq!(distributed_load.fem_a(&span), -60.0); // Expected value for fem_a
        assert_eq!(distributed_load.fem_b(&span), 60.0); // Expected value for fem_b
    }
}