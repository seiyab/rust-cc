use sourcecode::Position;

use token::TokenReader;

use parse::SyntaxTree;
use parse::Expression;

pub struct Root {
    expression: Expression,
}

impl Root {
    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

impl SyntaxTree for Root {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Root, (Option<Position>, String)> {
        Expression::parse(&mut token_reader)
        .map(|expression| Root {expression})
    }
}