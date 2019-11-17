use sourcecode::Position;

use token::TokenReader;

use parse::Equality;
use parse::SyntaxTree;

pub struct Expression {
    equality: Equality,
}

impl Expression {
    pub fn equality(&self) -> &Equality {
        &self.equality
    }
}

impl SyntaxTree for Expression {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Expression, (Option<Position>, String)> {
        Equality::parse(&mut token_reader)
        .map(|equality| Expression {equality})
    }
}