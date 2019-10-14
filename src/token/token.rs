#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Number(i64),
    Bracket(BracketSide)
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
    pub fn left_round_bracket() -> Token {
        Token::Bracket(BracketSide::Left(Bracket::Round))
    }
    pub fn right_round_bracket() -> Token {
        Token::Bracket(BracketSide::Right(Bracket::Round))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BracketSide {
    Left(Bracket),
    Right(Bracket),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bracket {
    Round,
}