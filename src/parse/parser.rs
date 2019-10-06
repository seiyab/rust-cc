use sourcecode::Position;
use sourcecode::Findable;

use token::Token;
use token::TokenReader;
use token::Operator;

use parse::SyntaxTree;
use parse::Expression;

pub fn parse(mut token_reader: &mut TokenReader) -> Result<SyntaxTree, (Option<Position>, String)> {
    parse_expression(&mut token_reader).map(|expression| SyntaxTree {expression})
}

fn parse_expression(token_reader: &mut TokenReader)
-> Result<Expression, (Option<Position>, String)> {
    let first_findable_token = match token_reader.next() {
        Some(findable_token) => findable_token,
        None => return Err((None, String::from("式を期待していましたが、トークンがありませんでした。"))),
    };
    let first_number = match first_findable_token.value() {
        &Token::Number(n) => n,
        _ => return Err((Some(first_findable_token.position()), String::from("数ではありません。"))),
    };
    let mut expression = Expression {
        head: first_findable_token.map(|_| first_number),
        tail: Vec::new(),
    };
    while let Some(findable_token) = token_reader.peek() {
        let token = findable_token.value();
        let operator = match token {
            &Token::Operator(operator) => operator,
            _ => break,
        };
        let add_or_sub = match operator {
            Operator::Add => Findable::new(operator, findable_token.position()),
            Operator::Sub => Findable::new(operator, findable_token.position()),
            _ => break,
        };
        token_reader.skip();
        let number = if let Some(findable_token) = token_reader.peek() {
            match findable_token.value() {
                &Token::Number(number) => Findable::new(number, findable_token.position()),
                _ => return Err((Some(findable_token.position()), String::from("数ではありません。"))),
            }
        } else {
            return Err((None, String::from("数を期待していましたが、トークンがありませんでした。")));
        };
        token_reader.skip();
        expression.tail.push((add_or_sub, number));
    }
    Ok(expression)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression() {
        // 3+5-1
        let findable_tokens = vec![
            Findable::new(Token::Number(3), Position(0)),
            Findable::new(Token::add(), Position(1)),
            Findable::new(Token::Number(5), Position(2)),
            Findable::new(Token::sub(), Position(3)),
            Findable::new(Token::Number(1), Position(4)),
        ];
        let mut token_reader = TokenReader::new(&findable_tokens);

        let syntax_tree = parse(&mut token_reader).unwrap();

        assert_eq!(syntax_tree.expression.head.value(), &3);

        assert_eq!(syntax_tree.expression.tail[0].0.value(), &Operator::Add);
        assert_eq!(syntax_tree.expression.tail[0].1.value(), &5);

        assert_eq!(syntax_tree.expression.tail[1].0.value(), &Operator::Sub);
        assert_eq!(syntax_tree.expression.tail[1].1.value(), &1);
    }
}