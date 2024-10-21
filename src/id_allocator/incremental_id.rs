pub trait IncremantalID {
    fn init() -> Self;
    fn get_and_increment(&mut self) -> Self;
}

impl IncremantalID for u32 {
    fn init() -> Self {
        0
    }

    fn get_and_increment(&mut self) -> Self {
        let new_id = *self;
        *self += 1;
        new_id
    }
}
