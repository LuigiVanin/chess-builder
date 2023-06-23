pub trait Builder<T> {
    fn build(self) -> T;
}

pub mod board_builder;
