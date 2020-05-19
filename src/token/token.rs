use std::collections::HashMap;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Operator(Operator),
    ReservedWord(ReservedWord),
    Number(i64),
    Bracket(BracketSide),
    Identifier(String),
    LineBreak,
    Comma,
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
    pub const fn assign() -> Token {
        Token::Operator(Operator::Assign)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
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
    Assign,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum ReservedWord {
    Let,
    Return,
    If,
    Then,
    Else,
    Func,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BracketSide {
    Left(Bracket),
    Right(Bracket),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bracket {
    Round,
    Curly,
}

pub struct Dictionary {
    terminal: Option<Token>,
    next: HashMap<char, Box<Dictionary>>,
}

impl Dictionary {
    fn of(string_to_token: &HashMap<String, Token>) -> Dictionary {
        let mut result = Dictionary{
            terminal: None,
            next: HashMap::new(),
        };
        for (s, token) in string_to_token {
            result.insert(s, token);
        }
        result
    }

    pub fn dig(&self, c: char) -> Result<&Dictionary, &Option<Token>> {
        self.next.get(&c).map(|d| d.as_ref()).ok_or(&self.terminal)
    }

    pub fn terminal(&self) -> &Option<Token> {
        &self.terminal
    }

    fn insert(&mut self, s: &String, t: &Token) {
        // csがmutなのが微妙なのでHaskell風のListを使いたい
        fn recursive_insert(dict: &mut Dictionary, cs: &mut Chars, t: &Token) {
            if let Some(c) = cs.next() {
                dict.next.entry(c).or_insert(Box::new(Dictionary {
                    terminal: None,
                    next: HashMap::new(),
                }));
                dict.next.entry(c).and_modify(|dict| recursive_insert(dict, cs, t));
            } else {
                dict.terminal = Some(t.clone());
            }
        }
        recursive_insert(self, &mut s.chars(), t);
    }

    pub fn default() -> Dictionary {
        let mut string_to_token = HashMap::new();
        string_to_token.insert(String::from("+"), Token::add());
        string_to_token.insert(String::from("-"), Token::sub());
        string_to_token.insert(String::from("*"), Token::mul());
        string_to_token.insert(String::from("/"), Token::div());
        string_to_token.insert(String::from("<"), Token::lt());
        string_to_token.insert(String::from("<="), Token::le());
        string_to_token.insert(String::from(">"), Token::gt());
        string_to_token.insert(String::from(">="), Token::ge());
        string_to_token.insert(String::from("=="), Token::eq());
        string_to_token.insert(String::from("!="), Token::neq());
        string_to_token.insert(String::from("("), Token::Bracket(BracketSide::Left(Bracket::Round)));
        string_to_token.insert(String::from(")"), Token::Bracket(BracketSide::Right(Bracket::Round)));
        string_to_token.insert(String::from("{"), Token::Bracket(BracketSide::Left(Bracket::Curly)));
        string_to_token.insert(String::from("}"), Token::Bracket(BracketSide::Right(Bracket::Curly)));
        string_to_token.insert(String::from(":="), Token::assign());

        Dictionary::of(&string_to_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dictionary = Dictionary::default();

        assert_eq!(Some(&Token::add()), dictionary.next.get(&'+').and_then(|d| d.terminal.as_ref()));

        let le = dictionary.next.get(&'<').and_then(|d| d.next.get(&'=')).and_then(|d| d.terminal.as_ref());
        assert_eq!(Some(&Token::le()), le);
    }
}