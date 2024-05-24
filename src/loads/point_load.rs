use crate::{nodes::node::SupportType, spans::{span::Span, traits::SpanDetails}};
use super::load_traits::LoadTraits;

#[derive(Debug, Clone, Copy, Default)]
pub struct PunctualLoad {
    pub value: f64,
    pub x: f64,
}

impl LoadTraits for PunctualLoad {
    fn fem_a(&self, span: &Span) -> f64 {
        let l = span.get_length();
        let a = self.x;
        let b = l - a;
        let w = self.value;

        match (span.get_est_a(), span.get_est_b()) {
            (SupportType::Fixed, SupportType::Fixed) => -w * a * b.powi(2) / l.powi(2),
            (SupportType::Fixed, SupportType::Roller) | (SupportType::Fixed, SupportType::Hinged) => {
                -w / l.powi(2) * (b.powi(2) * a + a.powi(2) * b / 2.0)
            }
            _ => 0.0,
        }
    }

    fn fem_b(&self, span: &Span) -> f64 {
        let l = span.get_length();
        let b = l - self.x;

        match (span.get_est_b(), span.get_est_a()) {
            (SupportType::Fixed, SupportType::Fixed) => b * self.x.powi(2) * self.value / l.powi(2),
            (SupportType::Fixed, SupportType::Roller) | (SupportType::Fixed, SupportType::Hinged) => {
                self.value / l.powi(2) * (b.powi(2) * self.x / 2.0 + self.x.powi(2) * b)
            }
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod punctual_load_test {
    use crate::{nodes::node::{Node, NodeTraits}, spans::traits::SpanBuilder};

    #[test]
    fn test_point_load_with_fixed_ends() {
        use super::*;
        let load = PunctualLoad { value: 40.0, x: 6.0 };

        let start_node = Node::new(0.0, 0.0, SupportType::Fixed);
        let end_node = Node::new(12.0, 0.0, SupportType::Fixed);

        let span = Span::new(start_node, end_node, vec![Default::default()], vec![Default::default()]);
        assert_eq!(-60.0, load.fem_a(&span));
        assert_eq!(60.0, load.fem_b(&span));
    }

    #[test]
    fn test2_point_load_with_fixed_ends() {
        use super::*;
        let load = PunctualLoad { value: 40.0, x: 2.0 };

        let start_node = Node::new(0.0, 0.0, SupportType::Fixed);
        let end_node = Node::new(4.0, 0.0, SupportType::Fixed);

        let span = Span::new(start_node, end_node, vec![Default::default()], vec![Default::default()]);
        assert_eq!(-20.0, load.fem_a(&span));
        assert_eq!(20.0, load.fem_b(&span));
    }

    #[test]
    fn test_point_load_with_fixed_end_and_free_end() {
        use super::*;
        let load = PunctualLoad { value: 40.0, x: 6.0 };

        let start_node = Node::new(0.0, 0.0, SupportType::Fixed);
        let end_node = Node::new(12.0, 0.0, SupportType::Hinged);

        let span = Span::new(start_node, end_node, vec![Default::default()], vec![Default::default()]);
        assert_eq!(-90.0, load.fem_a(&span));
        assert_eq!(0.0, load.fem_b(&span));
    }

    #[test]
    fn test_point_load_with_free_end_and_fixed_end() {
        use super::*;
        let load = PunctualLoad { value: 40.0, x: 6.0 };

        let start_node = Node::new(0.0, 0.0, SupportType::Hinged);
        let end_node = Node::new(12.0, 0.0, SupportType::Fixed);

        let span = Span::new(start_node, end_node, vec![Default::default()], vec![Default::default()]);
        assert_eq!(0.0, load.fem_a(&span));
        assert_eq!(90.0, load.fem_b(&span));
    }
}