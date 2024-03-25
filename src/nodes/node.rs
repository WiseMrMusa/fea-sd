#[derive(Debug, Clone, Copy)]
pub struct Node {
    x: f64,
    settlement: f64,
    support: SupportType
}

#[derive(Debug, Clone, Copy)]
pub enum SupportType {
    Fixed,
    Hinged,
    Roller
}

pub trait NodeTraits {
    fn new(x: f64, settlement: f64, support: SupportType) -> Self;
    fn get_x(&self) -> f64;
    fn get_support(&self) -> SupportType;
    fn get_deflection(&self) -> f64;
    fn get_rotation(&self) -> f64;
}

impl NodeTraits for Node {
    fn new(x: f64, settlement: f64, support: SupportType) -> Self {
        Node { x, settlement, support }
    }

    fn get_x(&self) -> f64 {
        self.x
    }

    fn get_support(&self) -> SupportType {
        self.support
    }

    fn get_deflection(&self) -> f64 {
        self.settlement
    }

    //TODO: Find a way to set rotation
    fn get_rotation(&self) -> f64 {
        0.0
    }
}