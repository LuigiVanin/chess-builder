#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveStatus {
    Check,
    Normal,
    Attack,
}

#[derive(Debug, Clone)]
pub struct Movement {
    pub src: (usize, usize),
    pub dst: (usize, usize),
    pub status: MoveStatus,
}

impl Movement {
    pub fn new(src: (usize, usize), dst: (usize, usize), status: MoveStatus) -> Movement {
        Movement {
            src,
            dst,
            status: status,
        }
    }
}
