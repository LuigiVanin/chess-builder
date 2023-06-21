#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PieceType {
    Horse,
    Hook,
    Bishop,
    Queen,
    King,
    Pun,
}

impl PieceType {
    pub fn point(self) -> u32 {
        match self {
            PieceType::Pun => 1,
            PieceType::Bishop => 3,
            PieceType::Horse => 3,
            PieceType::Hook => 5,
            PieceType::Queen => 10,
            PieceType::King => std::u32::MAX,
        }
    }

    pub fn unicode(self) -> char {
        match self {
            PieceType::Pun => '♟',
            PieceType::Bishop => 'B',
            PieceType::Horse => 'H',
            PieceType::Hook => 'R',
            PieceType::Queen => '♕',
            PieceType::King => 'K',
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PieceColor {
    White,
    Black,
}
