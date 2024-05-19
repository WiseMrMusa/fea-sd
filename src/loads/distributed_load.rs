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

// TODO: Not tested yet
impl LoadTraits for DistributedLoad {
    fn fem_a(&self, span: &Span) -> f64 {
        match span.get_est_a() {
            SupportType::Fixed => match span.get_est_b() {
                SupportType::Fixed => {
                    if self.start_value == self.end_value {
                        (self.start_value * span.get_length() * span.get_length()) / 12.0
                    } else {
                        let w = self.start_value - self.end_value;
                        let l = span.get_length();
                        (w * l * l) / 20.0
                            + (self.end_value * span.get_length() * span.get_length()) / 12.0
                    }
                }
                SupportType::Roller => {
                    if self.start_value == self.end_value {
                        (self.start_value * span.get_length() * span.get_length()) / 8.0
                    } else {
                        let w = self.start_value - self.end_value;
                        let l = span.get_length();
                        (w * l * l) / 20.0
                            + (self.end_value * span.get_length() * span.get_length()) / 15.0
                    }
                }
                SupportType::Hinged => 0.0,
            },
            SupportType::Roller => 0.0,
            SupportType::Hinged => 0.0,
        }
    }

    fn fem_b(&self, span: &Span) -> f64 {
        match span.get_est_a() {
            SupportType::Fixed => match span.get_est_b() {
                SupportType::Fixed => {
                    if self.start_value == self.end_value {
                        (self.start_value * span.get_length() * span.get_length()) / 12.0
                    } else {
                        let w = self.start_value - self.end_value;
                        let l = span.get_length();
                        (w * l * l) / 30.0 + (self.end_value * span.get_length() * span.get_length()) / 12.0
                    }
                }
                SupportType::Roller => {
                    0.0
                }
                SupportType::Hinged => 0.0,
            },
            SupportType::Roller => 0.0,
            SupportType::Hinged => 0.0,
        }
        
    }
}
