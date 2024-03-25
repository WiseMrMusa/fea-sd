use crate::{nodes::node::SupportType, spans::{span::Span, traits::SpanDetails}};

use super::load_traits::LoadTraits;

#[derive(Debug, Clone, Copy, Default)]
pub struct PunctualLoad {
    pub value: f64,
    pub x: f64
}

impl LoadTraits for PunctualLoad {

    fn fem_a (&self, span: &Span) -> f64 {
        let _span = span.get_length();
        let b = _span - self.x;

        match span.get_est_a() {
            SupportType::Fixed => {
                match span.get_est_b() {
                    SupportType::Fixed => {
                        (self.x * (b.powf(2.0))* self.value) / _span.powf(2.0)
                    }
                    SupportType::Roller => {
                        self.value / _span.powf(2.0) * (b.powf(2.0)*self.x  + self.x.powf(2.0)*b/2.0)
                    }
                    SupportType::Hinged => {
                        (self.value / _span.powf(2.0))* (b.powf(2.0)*self.x  + self.x.powf(2.0)*b/2.0)
                    }
                }
            }
            SupportType::Roller => {
                0.0
            }
            SupportType::Hinged => {
                0.0
            }
        }
    }
    
    fn fem_b(&self, span: &Span) -> f64 {
        let _span = span.get_length();
        let b = _span - self.x;

        match span.get_est_b() {
            SupportType::Fixed => {
                match span.get_est_a() {
                    SupportType::Fixed => {
                        (b * (self.x.powf(2.0))* self.value) / _span.powf(2.0)
                    }
                    SupportType::Roller => {
                        (self.value/ _span.powf(2.0)) * (b.powf(2.0)*self.x/2.0  + self.x.powf(2.0)*b)
                    }
                    SupportType::Hinged => {
                        (self.value/ _span.powf(2.0)) * (b.powf(2.0)*self.x/2.0  + self.x.powf(2.0)*b)
                    }
                }
            }
            SupportType::Roller => {
                0.0
            }
            SupportType::Hinged => {
                0.0
            }
        }
    }
    
}


#[cfg(test)]
mod punctual_load_test {
    use crate::{nodes::node::{Node, NodeTraits}, spans::traits::SpanBuilder};
    
    #[test]
    fn test_point_load_with_fixed_ends() {
        use super::*;
        let load = PunctualLoad{ value: 40.0, x: 6.0};

        let x = 0.0;
        let settlement = 0.0;
        let support = SupportType::Fixed;
        let start_node = Node::new(x, settlement, support);
        let end_node = Node::new(12.0, settlement, support);


        let span = Span::new(start_node, end_node, vec![Default::default()], vec![Default::default()]);
        assert_eq!(60.0, load.fem_a(&span));
        assert_eq!(60.0, load.fem_b(&span));
    }

    #[test]
    fn test_point_load_with_fixed_end_and_free_end() {
        use super::*;
        let load = PunctualLoad{ value: 40.0, x: 6.0};

        let x = 0.0;
        let settlement = 0.0;
        let fixed_support = SupportType::Fixed;
        let hinge_support = SupportType::Hinged;
        let start_node = Node::new(x, settlement, fixed_support);
        let end_node = Node::new(12.0, settlement, hinge_support);


        let span = Span::new(start_node, end_node, vec![Default::default()], vec![Default::default()]);
        assert_eq!(90.0, load.fem_a(&span));
        assert_eq!(0.0, load.fem_b(&span));
    }

    #[test]
    fn test_point_load_with_free_end_and_fixed_end() {
        use super::*;
        let load = PunctualLoad{ value: 40.0, x: 6.0};

        let x = 0.0;
        let settlement = 0.0;
        let fixed_support = SupportType::Fixed;
        let hinge_support = SupportType::Hinged;
        let start_node = Node::new(x, settlement, hinge_support);
        let end_node = Node::new(12.0, settlement, fixed_support);


        let span = Span::new(start_node, end_node, vec![Default::default()], vec![Default::default()]);
        assert_eq!(0.0, load.fem_a(&span));
        assert_eq!(90.0, load.fem_b(&span));
    }
}