use crate::{
    entities::{board::Board, movement::Movement, piece::Piece},
    utils::{Pattern, PieceMoveset},
};

use super::{utils::calc_moves_standart, PieceStrategy};

#[derive(Clone)]
pub struct KnightStrategy;

const KNIGHT_MOVEMENT_PATTERN: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];

impl PieceStrategy for KnightStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        let knight_moveset = PieceMoveset {
            moves: KNIGHT_MOVEMENT_PATTERN.to_vec(),
            pattern: Pattern::Once,
        };
        calc_moves_standart(piece, board, knight_moveset)
    }
}
