use std::fmt::Debug;
use std::fmt::{Formatter, Result};

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

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Piece {{ piece_type: {:?}, color: {:?}, pos: {:?} }}",
            self.piece_type.to_owned().unicode(),
            self.color,
            self.pos
        )
    }
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
