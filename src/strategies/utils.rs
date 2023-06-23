use crate::{
    entities::{
        board::Board,
        movement::{MoveStatus, Movement},
        piece::Piece,
    },
    utils::{Pattern, PieceMoveset},
};

pub fn is_bounded(x: i8, y: i8, board: &Board) -> bool {
    x >= 0 && x < board.width as i8 && y >= 0 && y < board.height as i8
}

fn define_movement(
    board: &Board,
    current_x: i8,
    current_y: i8,
    piece: &Piece,
) -> Option<MoveStatus> {
    match &board.tiles[current_x as usize][current_y as usize] {
        Some(found_piece) => {
            if found_piece.color != piece.color {
                return Some(MoveStatus::Attack);
            }
            return None;
        }
        None => {
            return Some(MoveStatus::Normal);
        }
    }
}

pub fn calc_moves_standart(
    piece: &Piece,
    board: &Board,
    piece_moveset: PieceMoveset,
) -> Vec<Movement> {
    let (src_x, src_y) = match piece.pos {
        None => return vec![],
        Some((x, y)) => (x, y),
    };

    let mut movements: Vec<Movement> = vec![];
    for (x, y) in piece_moveset.moves {
        let (mut current_x, mut current_y) = (src_x as i8, src_y as i8);
        while is_bounded(current_x + x, current_y + y, board) {
            current_x += x;
            current_y += y;
            let mov = define_movement(board, current_x, current_y, piece);
            match mov {
                Some(mov) => {
                    movements.push(Movement::new(
                        (src_x, src_y),
                        (current_x as usize, current_y as usize),
                        mov,
                    ));
                    if mov == MoveStatus::Attack {
                        break;
                    }
                }
                None => break,
            }
            if let Pattern::Once = piece_moveset.pattern {
                break;
            }
        }
    }
    return movements;
}
