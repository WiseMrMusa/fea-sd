pub trait FixedEndMomentsTrait {
    fn fem(&self) -> [f64; 2]{
        [self.fem_near(), self.fem_far()]
    }
    fn fem_near(&self) -> f64;
    fn fem_far(&self) -> f64;
}