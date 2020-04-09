use std::cmp::max;
use std::cmp::min;

use::general::SemiGroup;

use sourcecode::Position;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(line: usize, start_pos: usize, length: usize) -> Span {
        let start = Position{line, pos: start_pos};
        let end = Position{ line, pos: start_pos + length };
        Span { start, end }
    }
}

impl SemiGroup for Span {
    fn plus(&self, another: &Span) -> Span {
        Span {
            start: min(self.start, another.start),
            end: max(self.end, another.end),
        }
    }
}

pub struct Code<T> {
    pub value: T,
    pub span: Span,
}

impl <T> Code<T> {
    pub fn map<S, F>(&self, func: F) -> Code<S> 
    where F: Fn(&T) -> S {
        Code {
            value: func(&self.value),
            span: self.span,
        }
    }

    pub fn map_const<S>(&self, content: S) -> Code<S> {
        Code {
            value: content,
            span: self.span
        }
    }
}