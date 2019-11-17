use sourcecode::Position;

use token::TokenReader;

pub trait SyntaxTree: Sized {
    fn parse(token_reader: &mut TokenReader) -> Result<Self, (Option<Position>, String)>;
}