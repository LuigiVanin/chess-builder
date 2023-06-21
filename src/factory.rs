use std::collections::HashMap;

use crate::{
    entities::piece::Piece,
    strategies::{PunStrategy, QueenStrategy},
    utils::{PieceColor, PieceType, PieceType::*},
};

pub struct PieceFactory {
    impl_map: HashMap<PieceType, fn(PieceColor) -> Piece>,
}

impl PieceFactory {
    pub fn create_queen(color: PieceColor) -> Piece {
        Piece::new(Box::new(QueenStrategy), Queen, color)
    }

    pub fn create_pun(color: PieceColor) -> Piece {
        Piece::new(Box::new(PunStrategy), Pun, color)
    }

    pub fn new() -> Self {
        let mut map: HashMap<PieceType, fn(PieceColor) -> Piece> = HashMap::new();

        map.insert(PieceType::Queen, PieceFactory::create_queen);
        map.insert(PieceType::Pun, PieceFactory::create_pun);

        PieceFactory { impl_map: map }
    }

    pub fn create(&self, t: PieceType, c: PieceColor) -> Piece {
        let fun = self.impl_map.get(&t).unwrap();

        fun(c)
    }
}
