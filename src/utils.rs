#[derive(PartialEq, Eq, Hash)]
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
}
