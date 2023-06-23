use crate::{
    entities::{board::Board, movement::Movement, piece::Piece},
    strategies::utils::calc_moves_standart,
    utils::{Pattern, PieceMoveset},
};

use super::PieceStrategy;

#[derive(Clone)]
pub struct KingStrategy;

const KING_MOVEMENT_PATTERN: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

impl PieceStrategy for KingStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        let king_moveset = PieceMoveset {
            moves: KING_MOVEMENT_PATTERN.to_vec(),
            pattern: Pattern::Once,
        };
        calc_moves_standart(piece, board, king_moveset)
    }
}
