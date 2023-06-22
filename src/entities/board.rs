use super::{movement::Movement, piece::Piece};
use crate::{
    factory::PieceFactory,
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

    pub fn standart_board() -> Board {
        let mut board = Board::new(8, 8);
        let factory = PieceFactory::new();
        let queen = PieceFactory::create_queen(White);
        board.place_piece(queen, 3, 3);
        board.place_piece(factory.create(Pun, Black), 1, 0);
        board.place_piece(factory.create(Pun, Black), 1, 1);
        board.place_piece(factory.create(Pun, Black), 1, 2);
        board.place_piece(factory.create(Pun, Black), 1, 3);
        board.place_piece(factory.create(Pun, Black), 1, 4);
        // board.place_piece(factory.create(Pun, Black), 1, 5);
        board.place_piece(factory.create(Pun, Black), 1, 6);
        board.place_piece(factory.create(Pun, Black), 1, 7);

        board.place_piece(factory.create(Pun, White), 6, 3);

        board
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
        piece.strategy.calc_moveset(piece, self)
    }
}
