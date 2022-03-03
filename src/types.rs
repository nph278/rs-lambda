use thiserror::Error;

#[derive(Debug)]
pub enum Token {
    Lambda,
    Left,
    Right,
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Lambda(String, Box<Expression>),
    Application(Box<Expression>, Box<Expression>),
    Variable(String),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected character \"{0}\"")]
    UnexpectedChar(char),
    #[error("Unexpected token \"{0}\"")]
    UnexpectedToken(Token),
    #[error("Incomplete expression")]
    Incomplete,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Lambda => write!(f, "λ"),
            Token::Left => write!(f, "("),
            Token::Right => write!(f, ")"),
            Token::Variable(x) => write!(f, "{}", x),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Lambda(a, b) => write!(f, "λ{} {}", a, b),
            Expression::Application(a, b) => write!(f, "({} {})", a, b),
            Expression::Variable(a) => write!(f, "{}", a),
        }
    }
}
