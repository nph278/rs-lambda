use crate::types::{Expression, ParseError, Token};
use colored::Colorize;
use reglex::{lex, RuleList};
use std::iter::Peekable;

fn parse_one<T: Iterator<Item = Token>>(
    tokens: &mut Peekable<T>,
) -> Result<Expression, ParseError> {
    match tokens.next() {
        Some(Token::Variable(x)) => Ok(Expression::Variable(x)),
        Some(Token::Left) => {
            let a = parse_inner(tokens)?;
            match tokens.next() {
                Some(Token::Right) => Ok(a),
                Some(a) => Err(ParseError::UnexpectedToken(a)),
                None => Err(ParseError::Incomplete),
            }
        }
        Some(Token::Lambda) => match tokens.next() {
            Some(Token::Variable(a)) => {
                let b = parse_inner(tokens)?;
                Ok(Expression::Lambda(a, Box::new(b)))
            }
            Some(a) => Err(ParseError::UnexpectedToken(a)),
            None => Err(ParseError::Incomplete),
        },
        Some(a) => Err(ParseError::UnexpectedToken(a)),
        None => Err(ParseError::Incomplete),
    }
}

fn parse_inner<T: Iterator<Item = Token>>(
    tokens: &mut Peekable<T>,
) -> Result<Expression, ParseError> {
    let mut current = parse_one(tokens)?;

    loop {
        match tokens.peek() {
            Some(Token::Right) | None => return Ok(current),
            Some(_) => {
                current = Expression::Application(Box::new(current), Box::new(parse_one(tokens)?));
            }
        }
    }
}

pub fn parse(rules: &RuleList<Token>, input: &str, debug: bool) -> Result<Expression, ParseError> {
    let mut it = match lex(rules, input) {
        Ok(v) => {
            if debug {
                println!("{}", format!("{:?}", v).blue());
            }
            v.into_iter()
        }
        Err(i) => return Err(ParseError::UnexpectedChar(input.chars().nth(i).unwrap())),
    }
    .peekable();

    let res = parse_inner(&mut it);

    if let Some(x) = it.next() {
        Err(ParseError::UnexpectedToken(x))
    } else {
        if debug {
            println!("{}", format!("{:?}", res).blue());
        }
        res
    }
}
