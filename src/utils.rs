use crate::entities::movement::{MoveStatus, Movement};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PieceType {
    Knight,
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
            PieceType::Knight => 3,
            PieceType::Hook => 5,
            PieceType::Queen => 10,
            PieceType::King => std::u32::MAX,
        }
    }

    pub fn unicode(self) -> char {
        match self {
            // PieceType::Pun => '♟',
            PieceType::Pun => 'P',
            PieceType::Bishop => 'B',
            PieceType::Knight => 'H',
            PieceType::Hook => 'R',
            // PieceType::Queen => '♕',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum PieceColor {
    White,
    Black,
}

pub fn print_movement_map(moves: &Vec<Movement>) {
    let mut empty_mtx = vec![vec![0; 8]; 8];
    for m in moves {
        empty_mtx[m.src.0 as usize][m.src.1 as usize] = 5;
        empty_mtx[m.dst.0 as usize][m.dst.1 as usize] = match m.status {
            MoveStatus::Normal => 1,
            _ => 2,
        };
    }
    print!("  ");
    for idx in 0..8 {
        print!("{} ", idx);
    }
    println!();
    print!("  ");
    for _ in 0..8 {
        print!("__");
    }
    for (idx, row) in empty_mtx.iter().enumerate() {
        print!("\n{} |", idx);
        for item in row {
            print!(
                "{} ",
                match item {
                    0 => "□",
                    1 => "■",
                    2 => "✘",
                    5 => "●",
                    _ => " ",
                }
            );
        }
    }
}

pub fn is_valid_movement(movs: Vec<Movement>, dst: (usize, usize)) -> bool {
    for mov in movs {
        if mov.dst == dst {
            return true;
        }
    }
    return false;
}

pub enum Pattern {
    Inifinite,
    Once,
}

pub struct PieceMoveset {
    pub moves: Vec<(i8, i8)>,
    pub pattern: Pattern,
}
