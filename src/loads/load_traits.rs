use crate::spans::span::Span;

pub trait LoadTraits {
    fn fem_a(&self, span: &Span) -> f64;
    fn fem_b(&self, span: &Span) -> f64;
    fn fem(&self, span: &Span) -> [f64; 2]{
        [self.fem_a(span), self.fem_b(span)]
    }
}