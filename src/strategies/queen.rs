use crate::entities::{
    board::Board,
    movement::{MoveStatus, Movement},
    piece::Piece,
};

use super::{utils::is_bounded, PieceStrategy};

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
    fn calc_moveset(&self, piece: &Piece, board: &Board) -> Vec<Movement> {
        let (src_x, src_y) = match piece.pos {
            None => return vec![],
            Some((x, y)) => (x, y),
        };

        let mut movements: Vec<Movement> = vec![];
        for (x, y) in QUEEN_MOVEMENT_PATTERN {
            let (mut current_x, mut current_y) = (src_x as i8, src_y as i8);
            while is_bounded(current_x + x, current_y + y, board) {
                // TODO: Refactor this -> DRY
                current_x += x;
                current_y += y;
                match &board.tiles[current_x as usize][current_y as usize] {
                    Some(found_piece) => {
                        if found_piece.color != piece.color {
                            movements.push(Movement::new(
                                (src_x, src_y),
                                (current_x as usize, current_y as usize),
                                MoveStatus::Attack,
                            ));
                        }
                        break;
                    }
                    None => {
                        movements.push(Movement::new(
                            (src_x, src_y),
                            (current_x as usize, current_y as usize),
                            MoveStatus::Normal,
                        ));
                    }
                }
            }
        }
        return movements;
    }
}
