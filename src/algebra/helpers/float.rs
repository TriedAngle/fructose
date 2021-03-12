pub trait FloatBounded {
    const EPSILON: Self;
    const MAX_POSITIVE: Self;
    const MIN_POSITIVE: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;

    fn is_infinite(&self) -> bool;
    fn is_nan(&self) -> bool;
    fn is_sub_normal(&self) -> bool;
    fn is_normal(&self) -> bool;
    fn classify(&self) -> bool;
}
