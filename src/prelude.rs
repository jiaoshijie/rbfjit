pub enum OpsKind {
    Inc,          // +
    Dec,          // -
    Left,         // <
    Right,        // >
    Output,       // .
    Input,        // ,
    JmpIfZero,    // [
    JmpIfNonzero, // ]
    Nop,
}

impl From<u8> for OpsKind {
    fn from(value: u8) -> Self {
        return match value {
            43 => Self::Inc,          // +
            45 => Self::Dec,          // -
            60 => Self::Left,         // >
            62 => Self::Right,        // <
            46 => Self::Output,       // .
            44 => Self::Input,        // ,
            91 => Self::JmpIfZero,    // [
            93 => Self::JmpIfNonzero, // ]
            _ => Self::Nop,
        };
    }
}
