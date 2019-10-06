#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(char),
    Number(i64),
}