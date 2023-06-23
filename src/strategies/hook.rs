use crate::{
    entities::{board::Board, movement::Movement, piece::Piece},
    utils::{Pattern, PieceMoveset},
};

use super::{utils::calc_moves_standart, PieceStrategy};

#[derive(Clone)]
pub struct HookStrategy;

const HOOK_MOVEMENT_PATTERN: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl PieceStrategy for HookStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        let hook_moveset = PieceMoveset {
            moves: HOOK_MOVEMENT_PATTERN.to_vec(),
            pattern: Pattern::Inifinite,
        };
        calc_moves_standart(&piece, &board, hook_moveset)
    }
}
