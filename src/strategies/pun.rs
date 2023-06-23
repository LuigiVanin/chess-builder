use crate::entities::{board::Board, movement::Movement, piece::Piece};

use super::PieceStrategy;

#[derive(Clone)]

pub struct PunStrategy;

impl PieceStrategy for PunStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        println!("Pun moves");

        return vec![];
    }
}
