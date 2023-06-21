use crate::{
    strategies::PieceStrategy,
    utils::{PieceColor, PieceType},
};

#[derive(Clone)]
pub struct Piece {
    pub strategy: Box<dyn PieceStrategy>,
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub pos: Option<(usize, usize)>,
}

impl Piece {
    pub fn new(
        strategy: Box<dyn PieceStrategy>,
        piece_type: PieceType,
        color: PieceColor,
    ) -> Piece {
        Piece {
            strategy,
            piece_type,
            color,
            pos: None,
        }
    }
}
