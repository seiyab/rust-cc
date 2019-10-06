#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position(pub usize);

pub struct Findable<T> {
    value: T,
    position: Position,
}

impl <T> Findable<T> {
    pub fn new(value: T, position: Position) -> Findable<T> {
        Findable {
            value,
            position,
        }
    }

    pub fn value(self: &Self) -> &T {
        &self.value
    }

    pub fn position(self: &Self) -> Position {
        self.position
    }
}