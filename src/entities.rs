use crate::{strategies::PieceStrategy, utils::PieceType};

pub struct Piece {
    pub strategy: Box<dyn PieceStrategy>,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(strategy: Box<dyn PieceStrategy>, piece_type: PieceType) -> Piece {
        Piece {
            strategy,
            piece_type,
        }
    }
}

pub struct Board;

impl Board {
    fn move_piece(&self, piece: &Piece) {
        piece.strategy.moveset(piece, self);
    }
}

struct Game {
    board: Board,
    dead_pieces: [Vec<Piece>; 2], // First index is player with white pieces, second is player with black pieces
}
