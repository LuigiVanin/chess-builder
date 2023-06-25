use crate::{
    entities::{
        board::Board,
        movement::{MoveStatus, Movement},
        piece::{self, Piece},
    },
    strategies::utils::calc_moves_standart,
    utils::{Pattern, PieceColor, PieceMoveset},
};

use super::PieceStrategy;

#[derive(Clone)]

pub struct PunStrategy;

const PUN_MOVEMENT_PATTERN: [(i8, i8); 6] = [(1, 1), (1, -1), (1, 0), (-1, 0), (-1, 1), (-1, -1)];

impl PieceStrategy for PunStrategy {
    fn moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        // TODO: it need to test - and add the en passant(nightmare)
        let filtered_movset: Vec<(i8, i8)> = PUN_MOVEMENT_PATTERN
            .to_vec()
            .iter()
            .copied()
            .filter(|(x, _)| match piece.color {
                PieceColor::White => x < &0,
                PieceColor::Black => x > &0,
            })
            .collect();
        let pun_moves = PieceMoveset {
            moves: filtered_movset,
            pattern: Pattern::Once,
        };
        calc_moves_standart(piece, board, pun_moves)
            .iter()
            .cloned()
            .filter(|mov| {
                let (_, dst_y) = mov.dst;
                let (_, src_y) = mov.src;
                if dst_y == src_y {
                    if let MoveStatus::Normal = mov.status {
                        return true;
                    }
                    return false;
                }
                if let MoveStatus::Attack = mov.status {
                    return true;
                }
                return false;
            })
            .collect::<Vec<Movement>>()
    }
}
