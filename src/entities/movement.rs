pub enum MoveStatus {
    Check,
    Normal,
    Attack,
}

pub struct Movement {
    pub src: (u8, u8),
    pub dst: (u8, u8),
    pub status: MoveStatus,
}
