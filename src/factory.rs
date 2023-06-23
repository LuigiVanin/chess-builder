use std::collections::HashMap;

use crate::{
    entities::piece::Piece,
    strategies::{
        bishop::BishopStrategy, hook::HookStrategy, king::KingStrategy, knight::KnightStrategy,
        pun::PunStrategy, queen::QueenStrategy,
    },
    utils::{PieceColor, PieceType, PieceType::*},
};

pub struct PieceFactory {
    impl_map: HashMap<PieceType, fn(PieceColor) -> Piece>,
}

impl PieceFactory {
    pub fn Knight(color: PieceColor) -> Piece {
        Piece::new(Box::new(KnightStrategy), PieceType::Knight, color)
    }

    pub fn Queen(color: PieceColor) -> Piece {
        Piece::new(Box::new(QueenStrategy), PieceType::Queen, color)
    }

    pub fn Pun(color: PieceColor) -> Piece {
        Piece::new(Box::new(PunStrategy), PieceType::Pun, color)
    }

    pub fn Hook(color: PieceColor) -> Piece {
        Piece::new(Box::new(HookStrategy), PieceType::Hook, color)
    }

    pub fn Bishop(color: PieceColor) -> Piece {
        Piece::new(Box::new(BishopStrategy), PieceType::Bishop, color)
    }

    pub fn King(color: PieceColor) -> Piece {
        Piece::new(Box::new(KingStrategy), PieceType::King, color)
    }

    pub fn new() -> Self {
        let mut map: HashMap<PieceType, fn(PieceColor) -> Piece> = HashMap::new();

        map.insert(PieceType::Queen, PieceFactory::Queen);
        map.insert(PieceType::Pun, PieceFactory::Pun);
        map.insert(PieceType::Hook, PieceFactory::Hook);
        map.insert(PieceType::Bishop, PieceFactory::Bishop);
        map.insert(PieceType::King, PieceFactory::King);
        map.insert(PieceType::Knight, PieceFactory::Knight);
        PieceFactory { impl_map: map }
    }

    pub fn create(&self, t: PieceType, c: PieceColor) -> Piece {
        let fun = self.impl_map.get(&t).unwrap();

        fun(c)
    }
}
