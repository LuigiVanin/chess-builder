use super::{utils::calc_moves_standart, PieceStrategy};
use crate::utils::Pattern;
use crate::{
    entities::{board::Board, movement::Movement, piece::Piece},
    utils::PieceMoveset,
};

#[derive(Clone)]
pub struct QueenStrategy;

const QUEEN_MOVEMENT_PATTERN: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

impl PieceStrategy for QueenStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        let queen_moveset = PieceMoveset {
            moves: QUEEN_MOVEMENT_PATTERN.to_vec(),
            pattern: Pattern::Inifinite,
        };

        calc_moves_standart(piece, board, queen_moveset)
    }
}
