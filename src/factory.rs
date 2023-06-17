use std::collections::HashMap;

use crate::{entities::Piece, strategies::QueenStrategy, utils::PieceType};

pub struct PieceFactory {
    impl_map: HashMap<PieceType, fn() -> Piece>,
}

impl PieceFactory {
    pub fn create_queen() -> Piece {
        Piece::new(Box::new(QueenStrategy::new()), PieceType::Queen)
    }

    pub fn new(self) -> Self {
        let mut map: HashMap<PieceType, fn() -> Piece> = HashMap::new();

        map.insert(PieceType::Queen, PieceFactory::create_queen);

        PieceFactory { impl_map: map }
    }

    pub fn create(self, t: PieceType) -> Piece {
        let fun = self.impl_map.get(&t).unwrap();

        fun()
    }
}
