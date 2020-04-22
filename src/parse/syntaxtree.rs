use general::TryReader;

use token::Token;

use sourcecode::Span;
use sourcecode::Code;

pub trait SyntaxTree: Sized {
    fn parse(token_reader: &mut TryReader<Code<Token>>) -> Result<Self, (Option<Span>, String)>;
    fn span(&self) -> Span;
}