use std::rc::Rc;

use crate::utils::PieceColor::*;
use crate::{
    entities::{
        board::Board,
        movement::{MoveStatus, Movement},
        piece::Piece,
    },
    helpers::{board_builder::BoardBuilder, Builder},
};

pub struct Chess {
    board: Board,
    dead_pieces: [Vec<Piece>; 2], // First index is player with white pieces, second is player with black pieces
    movement_cache: Vec<Rc<Movement>>,
    movement_history: Vec<Movement>,
}

impl Chess {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            dead_pieces: [Vec::new(), Vec::new()],
            movement_cache: Vec::new(),
            movement_history: Vec::new(),
        }
    }

    pub fn StandartBoard() -> Self {
        Chess::new(
            BoardBuilder::new(8, 8)
                .add_pun((1, 0), Black)
                .add_pun((1, 1), Black)
                .add_pun((1, 2), Black)
                .add_pun((1, 3), Black)
                .add_pun((1, 4), Black)
                .add_pun((1, 5), Black)
                .add_pun((1, 6), Black)
                .add_pun((1, 7), Black)
                .add_hook((0, 0), Black)
                .add_hook((0, 7), Black)
                .add_knight((0, 1), Black)
                .add_knight((0, 6), Black)
                .add_bishop((0, 2), Black)
                .add_bishop((0, 5), Black)
                .add_queen((0, 3), Black)
                .add_king((0, 4), Black)
                .add_pun((6, 0), White)
                .add_pun((6, 1), White)
                .add_pun((6, 2), White)
                .add_pun((6, 3), White)
                .add_pun((6, 4), White)
                .add_pun((6, 5), White)
                .add_pun((6, 6), White)
                .add_pun((6, 7), White)
                .add_hook((7, 0), White)
                .add_hook((7, 7), White)
                .add_knight((7, 1), White)
                .add_knight((7, 6), White)
                .add_bishop((7, 2), White)
                .add_bishop((7, 5), White)
                .add_queen((7, 4), White)
                .add_king((7, 3), White)
                .build(),
        )
    }

    pub fn TestBoard() -> Self {
        Self::new(Board::test_board())
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Piece> {
        self.board.get_tile(x, y)
    }

    pub fn print_board(&self) {
        self.board.print_board();
    }

    pub fn possible_moves(&self, piece: &Piece) -> Vec<Movement> {
        self.board.possible_moves(piece)
    }
}
