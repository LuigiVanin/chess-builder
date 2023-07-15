use std::rc::Rc;

use crate::utils::is_valid_movement;
use crate::utils::PieceColor::{self, *};
use crate::{
    entities::{board::Board, movement::Movement, piece::Piece},
    helpers::{board_builder::BoardBuilder, Builder},
};

pub struct Player {
    color: PieceColor,
    name: String,
}

pub struct Chess {
    board: Board,
    dead_pieces: [Vec<Piece>; 2], // First index is player with white pieces, second is player with black pieces
    movement_cache: Vec<Rc<Movement>>,
    movement_history: Vec<Movement>,
    pub players: [Player; 2],
}

impl Chess {
    pub fn new(board: Board, player_1: Player, player_2: Player) -> Self {
        Self {
            board,
            dead_pieces: [Vec::new(), Vec::new()],
            movement_cache: Vec::new(),
            movement_history: Vec::new(),
            players: [player_1, player_2],
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
            Player {
                color: White,
                name: String::from("Player 1"),
            },
            Player {
                color: Black,
                name: String::from("Player 2"),
            },
        )
    }

    pub fn TestBoard() -> Self {
        Self::new(
            Board::test_board(),
            Player {
                color: White,
                name: String::from("Player 1"),
            },
            Player {
                color: Black,
                name: String::from("Player 2"),
            },
        )
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

    pub fn move_piece(
        mut self,
        src: (usize, usize),
        dst: (usize, usize),
        player: Player,
    ) -> Result<Option<Piece>, &'static str> {
        let src_tile = self.board.get_tile(src.0, src.1);
        if let Some(piece) = src_tile {
            if piece.color == player.color {
                let movs = self.possible_moves(piece);
                if !is_valid_movement(movs, dst) {
                    return Err("Invalid movement");
                }
                let removed_piece = self.board.place_piece(piece.clone(), dst.0, dst.1);
                self.board.take_piece(src.0, src.1);
            } else {
                return Err("Not your piece");
            }
        } else {
            return Err("No piece found");
        }
        Err("Not implemented")
    }
}
