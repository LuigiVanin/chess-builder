use crate::{
    entities::{board::Board, movement::Movement, piece::Piece},
    utils::{Pattern, PieceMoveset},
};

use super::{utils::calc_moves_standart, PieceStrategy};

#[derive(Clone)]
pub struct BishopStrategy;

const BISHOP_MOVEMENT_PATTERN: [(i8, i8); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];

impl PieceStrategy for BishopStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        let bishop_moveset = PieceMoveset {
            moves: BISHOP_MOVEMENT_PATTERN.to_vec(),
            pattern: Pattern::Inifinite,
        };

        calc_moves_standart(piece, board, bishop_moveset)
    }
}
