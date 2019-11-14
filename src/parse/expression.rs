use sourcecode::Position;

use token::TokenReader;

use parse::Equality;

pub struct Expression {
    equality: Equality,
}

impl Expression {
    pub fn parse(mut token_reader: &mut TokenReader)
    -> Result<Expression, (Option<Position>, String)> {
        Equality::parse(&mut token_reader)
        .map(|equality| Expression {equality})
    }

    pub fn equality(&self) -> &Equality {
        &self.equality
    }
}