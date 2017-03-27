use ast::*;
use scanner::{Token, TokenWithContext};
use std::iter::Peekable;

pub fn parse(tokens: Vec<TokenWithContext>) -> Expr {
    let mut iter = tokens.iter().peekable();
    parse_expression(&mut iter)
}

fn parse_expression<'a, I>(tokens: &mut Peekable<I>) -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    parse_equality(tokens)
}

fn parse_binary<'a, I>(tokens: &mut Peekable<I>,
                       map_operator: &Fn(&Token) -> Option<Operator>,
                       parse_subexpression: &Fn(&mut Peekable<I>) -> Expr)
                       -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    let mut expr;
    {
        expr = parse_subexpression(tokens);
    };
    let peeked_token;
    {
        peeked_token = tokens.peek().cloned(); // Can I avoid this?
    };
    while let Some(peeked_token) = peeked_token {
        if let Some(mapped_operator) = map_operator(&peeked_token.token) {
            {
                // Just advance, we know all we need from the peeked value
                let _ = tokens.next();
            }
            let right;
            {
                right = parse_subexpression(tokens);
            };
            let binary_expression = BinaryExpr {
                left: expr,
                operator: mapped_operator,
                right: right,
            };
            expr = Expr::Binary(Box::new(binary_expression));
        } else {
            break;
        }
    }
    expr
}

fn parse_equality<'a, I>(tokens: &mut Peekable<I>) -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    fn map_operator(token: &Token) -> Option<Operator> {
        match token {
            &Token::BangEqual => Some(Operator::NotEqual),
            &Token::EqualEqual => Some(Operator::Equal),
            _ => None,
        }
    }
    parse_binary(tokens, &map_operator, &parse_comparison)
}

fn parse_comparison<'a, I>(tokens: &mut Peekable<I>) -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    fn map_operator(token: &Token) -> Option<Operator> {
        match token {
            &Token::Greater => Some(Operator::Greater),
            &Token::GreaterEqual => Some(Operator::GreaterEqual),
            &Token::Less => Some(Operator::Less),
            &Token::LessEqual => Some(Operator::LessEqual),
            _ => None,
        }
    }
    parse_binary(tokens, &map_operator, &parse_term)
}

fn parse_term<'a, I>(tokens: &mut Peekable<I>) -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    fn map_operator(token: &Token) -> Option<Operator> {
        match token {
            &Token::Minus => Some(Operator::Minus),
            &Token::Plus => Some(Operator::Plus),
            _ => None,
        }
    }
    parse_binary(tokens, &map_operator, &parse_factor)
}

fn parse_factor<'a, I>(tokens: &mut Peekable<I>) -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    fn map_operator(token: &Token) -> Option<Operator> {
        match token {
            &Token::Slash => Some(Operator::Slash),
            &Token::Star => Some(Operator::Star),
            _ => None,
        }
    }
    parse_binary(tokens, &map_operator, &parse_unary)
}

fn parse_unary<'a, I>(tokens: &mut Peekable<I>) -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    fn map_operator(token: &Token) -> Option<Operator> {
        match token {
            &Token::Minus => Some(Operator::Minus),
            &Token::Bang => Some(Operator::Bang),
            _ => None,
        }
    }
    let peeked_token;
    {
        peeked_token = tokens.peek().cloned(); // Can I avoid this?
    };
    if let Some(Some(mapped_operator)) =
        peeked_token.map(|pt| &pt.token)
            .map(map_operator) {
        {
            // Just advance, we know all we need from the peeked value
            let _ = tokens.next();
        }
        let right;
        {
            right = parse_unary(tokens);
        };
        let unary_expression = UnaryExpr {
            operator: mapped_operator,
            right: right,
        };
        return Expr::Unary(Box::new(unary_expression));
    } else {
        parse_primary(tokens)
    }
}

fn parse_primary<'a, I>(tokens: &mut Peekable<I>) -> Expr
    where I: Iterator<Item = &'a TokenWithContext>
{
    let primary_token;
    {
        primary_token = tokens.next();
    };
    if let Some(primary_token) = primary_token {
        match primary_token.token {
            Token::False => Expr::Literal(Literal::BoolLiteral(false)),
            Token::True => Expr::Literal(Literal::BoolLiteral(true)),
            Token::Nil => Expr::Literal(Literal::NilLiteral),
            Token::NumberLiteral(n) => Expr::Literal(Literal::NumberLiteral(n)),
            Token::StringLiteral(ref s) => Expr::Literal(Literal::StringLiteral(s.clone())),
            Token::LeftParen => {
                let expr;
                {
                    expr = parse_expression(tokens);
                };
                {
                    if let Some(token) = tokens.next() {
                        if token.token == Token::LeftParen {
                            let grouping_expression = Grouping { expr: expr };
                            return Expr::Grouping(Box::new(grouping_expression));
                        }
                    }
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        }
    } else {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use scanner::*;
    use parser::*;
    use pretty_printer::PrettyPrint;

    #[test]
    fn literal() {
        let tokens = scan(&"123".into()).unwrap();
        let expr = parse(tokens);
        assert_eq!("123", &expr.pretty_print());
    }

    #[test]
    fn binary() {
        let tokens = scan(&"123+456".into()).unwrap();
        let expr = parse(tokens);
        assert_eq!("123 + 456", &expr.pretty_print());
    }
}