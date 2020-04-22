use general::FreeMonoid;
use general::SemiGroup;
use general::TryReader;

use token::Token;

use sourcecode::Code;
use sourcecode::Span;

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
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
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