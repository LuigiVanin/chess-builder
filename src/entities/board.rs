use std::iter::Enumerate;

use super::piece::Piece;
use crate::{
    factory::PieceFactory,
    utils::{PieceColor::*, PieceType::*},
};

pub struct Board {
    tiles: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new(x: usize, y: usize) -> Board {
        let tiles = vec![vec![None; y]; x];

        Board { tiles }
    }

    pub fn standart_board() -> Board {
        let mut board = Board::new(8, 8);
        let factory = PieceFactory::new();
        let queen = PieceFactory::create_queen(White);
        board.place_piece(queen, 3, 3);
        board.place_piece(factory.create(Pun, White), 1, 0);
        board.place_piece(factory.create(Pun, White), 1, 1);
        board.place_piece(factory.create(Pun, White), 1, 2);
        board.place_piece(factory.create(Pun, White), 1, 3);

        board
    }

    pub fn print_board(self) -> () {
        print!("  ");
        for idx in 0..self.tiles.len() {
            print!("{} ", idx);
        }
        println!("");
        for (idx, row) in self.tiles.into_iter().enumerate() {
            print!("{} ", idx);
            for tile in row {
                match tile {
                    Some(piece) => print!("{} ", piece.piece_type.unicode()),
                    None => print!("0 "),
                }
            }
            println!();
        }
    }

    pub fn place_piece(&mut self, mut piece: Piece, x: usize, y: usize) -> Option<Piece> {
        let mut old_piece = self.tiles[x][y].take();
        piece.pos = Some((x, y));
        self.tiles[x][y] = Some(piece);

        if let Some(p) = old_piece.as_mut() {
            p.pos = None;
        }
        old_piece
    }

    fn move_piece(&self, piece: &Piece) {
        piece.strategy.moveset(piece, self);
    }
}
