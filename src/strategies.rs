use crate::entities::{board::Board, movement::Movement, piece::Piece};

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
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement>;
}

impl Clone for Box<dyn PieceStrategy> {
    fn clone(&self) -> Box<dyn PieceStrategy> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct QueenStrategy;

impl PieceStrategy for QueenStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        println!("Queen moves");

        return vec![];
    }
}

#[derive(Clone)]

pub struct PunStrategy;

impl PieceStrategy for PunStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        println!("Pun moves");

        return vec![];
    }
}
