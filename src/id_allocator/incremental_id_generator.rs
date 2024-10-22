use ::num::traits::Num;

pub trait IncremantalIDGenerator {
    type ID: Copy;

    fn init() -> Self;
    fn get_and_increment(&mut self) -> Self::ID;
}

impl<T: Copy + Num> IncremantalIDGenerator for T {
    type ID = T;

    fn init() -> Self {
        Self::zero()
    }

    fn get_and_increment(&mut self) -> Self {
        let new_id = *self;
        *self = *self + Self::one();
        new_id
    }
}
