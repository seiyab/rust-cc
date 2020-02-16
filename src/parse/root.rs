use sourcecode::Position;

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
    -> Result<Root, (Option<Position>, String)> {
        let mut statements = Vec::new();
        while token_reader.has_next() {
            match Statement::parse(&mut token_reader) {
                Ok(statement) => statements.push(statement),
                Err(err) => return Err(err),
            }
        }
        Ok(Root{statements})
    }
}