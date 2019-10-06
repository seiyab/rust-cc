use sourcecode::Findable;

use token::Operator;

pub struct SyntaxTree {
    pub expression: Expression,
}

pub struct Expression {
    pub head: Findable<i64>,
    pub tail: Vec<(Findable<Operator>, Findable<i64>)>
}