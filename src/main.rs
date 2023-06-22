mod entities;
mod factory;
// mod strategies_2;
mod strategies;
mod utils;

use entities::{board::Board, piece::Piece};

use crate::{
    entities::movement::MoveStatus,
    factory::PieceFactory,
    utils::{print_movement_table, PieceColor::*, PieceType::*},
};

struct Game {
    board: Board,
    dead_pieces: [Vec<Piece>; 2], // First index is player with white pieces, second is player with black pieces
}

fn main() {
    let mut chess = Game {
        board: Board::standart_board(),
        dead_pieces: [Vec::new(), Vec::new()],
    };

    chess.board.print_board();
    let opt_piece = chess.board.get_tile(3, 3);
    println!("{:?}", opt_piece);
    if let Some(piece) = opt_piece {
        let mov = chess.board.possible_moves(piece);
        print_movement_table(&mov);
    }
}
