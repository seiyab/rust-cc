use sourcecode::Position;

use token::TokenReader;

use parse::Relational;

pub struct Expression {
    relational: Relational,
}

impl Expression {
    pub fn parse(mut token_reader: &mut TokenReader)
    -> Result<Expression, (Option<Position>, String)> {
        Relational::parse(&mut token_reader)
        .map(|relational| Expression {relational})
    }

    pub fn relational(&self) -> &Relational {
        &self.relational
    }
}