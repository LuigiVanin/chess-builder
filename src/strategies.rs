use crate::entities::{Board, Piece};

pub trait PieceStrategy {
    fn moveset(&self, piece: &Piece, board: &Board);
}

pub struct QueenStrategy;

impl QueenStrategy {
    pub fn new() -> QueenStrategy {
        QueenStrategy
    }
}

impl PieceStrategy for QueenStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) {
        println!("Queen moves");
    }
}
