mod entities;
mod factory;
mod strategies;
mod utils;

use entities::{board::Board, piece::Piece};

use crate::{
    factory::PieceFactory,
    utils::{PieceColor::*, PieceType::*},
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
    println!("Hello, world!");
}
