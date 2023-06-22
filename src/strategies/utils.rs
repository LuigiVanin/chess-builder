use crate::entities::{board::Board, movement::Movement, piece::Piece};

pub fn is_bounded(x: i8, y: i8, board: &Board) -> bool {
    x >= 0 && x < board.width as i8 && y >= 0 && y < board.height as i8
}
