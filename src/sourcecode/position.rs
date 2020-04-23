#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub pos: usize,
}
