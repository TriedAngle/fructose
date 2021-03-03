pub trait Operator: Copy {}

#[derive(Copy, Clone)]
pub struct Additive {}

#[derive(Copy, Clone)]
pub struct Multiplicative {}

impl Operator for Additive {}

impl Operator for Multiplicative {}
