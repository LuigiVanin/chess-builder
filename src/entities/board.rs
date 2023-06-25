use super::{movement::Movement, piece::Piece};
use crate::{
    factory::PieceFactory,
    helpers::{board_builder::BoardBuilder, Builder},
    utils::{PieceColor::*, PieceType::*},
};

pub struct Board {
    pub tiles: Vec<Vec<Option<Piece>>>,
    pub height: usize,
    pub width: usize,
}

impl Board {
    pub fn new(x: usize, y: usize) -> Board {
        let tiles = vec![vec![None; y]; x];

        Board {
            tiles,
            height: y,
            width: x,
        }
    }

    pub fn test_board() -> Board {
        BoardBuilder::new(8, 8)
            .add_pun((1, 0), Black)
            .add_pun((1, 1), Black)
            .add_pun((1, 2), Black)
            .add_pun((1, 3), Black)
            .add_pun((1, 4), Black)
            .add_pun((1, 5), Black)
            .add_pun((1, 6), Black)
            .add_pun((1, 7), Black)
            .add_pun((6, 3), White)
            .add_pun((6, 6), White)
            .add_pun((5, 6), White)
            .add_pun((2, 2), White)
            .add_pun((4, 7), Black)
            .add_knight((5, 4), Black)
            .add_pun((3, 5), Black)
            .add_queen((3, 3), White)
            .build()
    }

    pub fn print_board(&self) -> () {
        print!("  ");
        for idx in 0..self.tiles.len() {
            print!("{} ", idx);
        }
        println!("");
        for (idx, row) in self.tiles.iter().enumerate() {
            print!("{} ", idx);
            for tile in row {
                match tile {
                    Some(piece) => print!("{} ", piece.to_owned().piece_type.unicode()),
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

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Piece> {
        self.tiles[x][y].as_ref()
    }

    pub fn possible_moves(&self, piece: &Piece) -> Vec<Movement> {
        piece.strategy.moveset(piece, self)
    }
}
