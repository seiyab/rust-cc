#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Number(i64),
}

impl Token {
    pub fn add() -> Token {
        Token::Operator(Operator::Add)
    }
    pub fn sub() -> Token {
        Token::Operator(Operator::Sub)
    }
    pub fn mul() -> Token {
        Token::Operator(Operator::Mul)
    }
    pub fn div() -> Token {
        Token::Operator(Operator::Div)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}