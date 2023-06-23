use crate::utils::PieceType::*;
use crate::{
    entities::{board::Board, piece::Piece},
    factory::PieceFactory,
    utils::PieceColor,
};

use super::Builder;

pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            board: Board::new(h, w),
        }
    }

    pub fn add_piece(mut self, piece: Piece, x: usize, y: usize) -> Self {
        self.board.place_piece(piece, x, y);
        self
    }

    pub fn add_knight(mut self, coord: (usize, usize), color: PieceColor) -> Self {
        self.board
            .place_piece(PieceFactory::Knight(color), coord.0, coord.1);
        self
    }

    pub fn add_pun(mut self, coord: (usize, usize), color: PieceColor) -> Self {
        self.board
            .place_piece(PieceFactory::Pun(color), coord.0, coord.1);
        self
    }

    pub fn add_bishop(mut self, coord: (usize, usize), color: PieceColor) -> Self {
        self.board
            .place_piece(PieceFactory::Bishop(color), coord.0, coord.1);
        self
    }

    pub fn add_queen(mut self, coord: (usize, usize), color: PieceColor) -> Self {
        self.board
            .place_piece(PieceFactory::Queen(color), coord.0, coord.1);
        self
    }

    pub fn add_hook(mut self, coord: (usize, usize), color: PieceColor) -> Self {
        self.board
            .place_piece(PieceFactory::Hook(color), coord.0, coord.1);
        self
    }

    pub fn add_king(mut self, coord: (usize, usize), color: PieceColor) -> Self {
        self.board
            .place_piece(PieceFactory::King(color), coord.0, coord.1);
        self
    }
}

impl Builder<Board> for BoardBuilder {
    fn build(self) -> Board {
        self.board
    }
}
