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
    pub const fn eq() -> Token {
        Token::Operator(Operator::Equal)
    }
    pub const fn neq() -> Token {
        Token::Operator(Operator::NotEqual)
    }
    pub const fn lt() -> Token {
        Token::Operator(Operator::Less)
    }
    pub const fn gt() -> Token {
        Token::Operator(Operator::Greater)
    }
    pub const fn le() -> Token {
        Token::Operator(Operator::LessEq)
    }
    pub const fn ge() -> Token {
        Token::Operator(Operator::GreaterEq)
    }
    pub const fn left_round_bracket() -> Token {
        Token::Bracket(BracketSide::Left(Bracket::Round))
    }
    pub const fn right_round_bracket() -> Token {
        Token::Bracket(BracketSide::Right(Bracket::Round))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEq,
    GreaterEq,
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