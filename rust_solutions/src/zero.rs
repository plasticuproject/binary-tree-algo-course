pub trait Zero {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}
