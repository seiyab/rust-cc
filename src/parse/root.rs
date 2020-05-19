use general::FreeMonoid;
use general::SemiGroup;
use general::TryReader;

use token::Token;

use sourcecode::Code;
use sourcecode::Span;

use parse::SyntaxTree;
use parse::Func;

pub struct Root {
    pub funcs: Vec<Func>,
}


impl SyntaxTree for Root {
    fn parse(mut token_reader: &mut TryReader<Code<Token>>)
    -> Result<Root, (Option<Span>, String)> {
        let mut funcs = Vec::new();
        while token_reader.has_next() {
            match Func::parse(&mut token_reader) {
                Ok(func) => funcs.push(func),
                Err(err) => return Err(err),
            }
            token_reader.drop_while(|token| token.value == Token::LineBreak);
        }
        Ok(Root{funcs})
    }

    fn span(&self) -> Span {
        self.funcs
            .iter()
            .map(|stmt| stmt.span())
            .map(FreeMonoid::Some)
            .fold(FreeMonoid::Zero, |acc, x| acc.plus(&x))
            .get()
            .unwrap()
            .clone()
    }
}