use general::FreeMonoid;
use general::SemiGroup;

use sourcecode::Span;

use token::TokenReader;

use parse::SyntaxTree;
use parse::Statement;

pub struct Root {
    statements: Vec<Statement>,
}

impl Root {
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl SyntaxTree for Root {
    fn parse(mut token_reader: &mut TokenReader)
    -> Result<Root, (Option<Span>, String)> {
        let mut statements = Vec::new();
        while token_reader.has_next() {
            match Statement::parse(&mut token_reader) {
                Ok(statement) => statements.push(statement),
                Err(err) => return Err(err),
            }
        }
        Ok(Root{statements})
    }

    fn span(&self) -> Span {
        self.statements
            .iter()
            .map(|stmt| stmt.span())
            .map(FreeMonoid::Some)
            .fold(FreeMonoid::Zero, |acc, x| acc.plus(&x))
            .get()
            .unwrap()
            .clone()
    }
}