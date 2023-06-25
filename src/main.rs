mod entities;
mod factory;
// mod strategies_2;
mod chess;
mod helpers;
mod strategies;
mod utils;

use chess::Chess;
use entities::{board::Board, piece::Piece};

use crate::utils::print_movement_map;

fn main() {
    // let mut chess = Game {
    //     board: Board::standart_board(),
    //     dead_pieces: [Vec::new(), Vec::new()],
    // };

    let chess = Chess::TestBoard();

    chess.print_board();
    let opt_piece = chess.get_tile(3, 3);

    if let Some(piece) = opt_piece {
        let mov = chess.possible_moves(piece);
        print_movement_map(&mov);
    }

    let opt_king = chess.get_tile(2, 2);
    if let Some(king) = opt_king {
        let mov = chess.possible_moves(king);
        print_movement_map(&mov);
    }

    let opt_knight = chess.get_tile(5, 4);
    if let Some(knight) = opt_knight {
        let mov = chess.possible_moves(knight);
        print_movement_map(&mov);
    }

    let opt_pun = chess.get_tile(5, 6);
    if let Some(pun) = opt_pun {
        let mov = chess.possible_moves(pun);
        print_movement_map(&mov);
    } else {
        println!("No pun found");
    }
}
