use sourcecode::Position;

use token::TokenReader;

use parse::Expression;

pub struct SyntaxTree {
    expression: Expression,
}

impl SyntaxTree {
    pub fn parse(mut token_reader: &mut TokenReader)
    -> Result<SyntaxTree, (Option<Position>, String)> {
        Expression::parse(&mut token_reader)
        .map(|expression| SyntaxTree {expression})
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}