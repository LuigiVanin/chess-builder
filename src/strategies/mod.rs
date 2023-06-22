use crate::entities::{board::Board, movement::Movement, piece::Piece};

pub mod pun;
pub mod queen;
pub mod utils;

pub trait PieceStrategyClone {
    fn clone_box(&self) -> Box<dyn PieceStrategy>;
}

impl<T> PieceStrategyClone for T
where
    T: 'static + PieceStrategy + Clone,
{
    fn clone_box(&self) -> Box<dyn PieceStrategy> {
        Box::new(self.clone())
    }
}

pub trait PieceStrategy: PieceStrategyClone {
    fn calc_moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement>;
}

impl Clone for Box<dyn PieceStrategy> {
    fn clone(&self) -> Box<dyn PieceStrategy> {
        self.clone_box()
    }
}
