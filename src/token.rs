use std::fmt;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum TokenKind {
    DoubleEqual,
    Else,
    Less,
    LessOrEqual,
    Comma,
    Break,
    True,
    False,
    NotEqual,
    Percent,
    LeftParen,
    LCurly,
    RCurly,
    LBrace,
    RBrace,
    While,
    If,
    RightParen,
    Slash,
    Plus,
    Minus,
    Asterisk,
    Equal,
    Arrow,
    Exclamation,
    Or,
    And,
    Greater,
    GreaterOrEqual,
    Colon,
    String(String),
    Identifier(String),
    Numeric(f64),
    Load,
    Extern,
    Eof,
    End,
    Fn,
    Struct,
    Return,
    Dot,
    Pipe,
    DoubleColon
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
}

fn token_name(token: &TokenKind) -> &str {
    match token {
        TokenKind::NotEqual => "NotEqual",
        TokenKind::LeftParen => "LeftParen",
        TokenKind::RightParen => "RightParen",
        TokenKind::Slash => "Slash",
        TokenKind::Identifier { .. } => "Identifier",
        TokenKind::Numeric { .. } => "Numeric",
        TokenKind::Plus => "Plus",
        TokenKind::Minus => "Minus",
        TokenKind::Asterisk => "Asterisk",
        TokenKind::Equal => "Equal",
        TokenKind::LCurly => "LCurly",
        TokenKind::RCurly => "RCurly",
        TokenKind::LBrace => "LBrace",
        TokenKind::RBrace => "RBrace",
        TokenKind::If => "If",
        TokenKind::While => "While",
        TokenKind::True => "True",
        TokenKind::False => "False",
        TokenKind::DoubleEqual => "DoubleEqual",
        TokenKind::Percent => "Percent",
        TokenKind::Exclamation => "Exclamation",
        TokenKind::Break => "Break",
        TokenKind::String { .. } => "String",
        TokenKind::Eof => "Eof",
        TokenKind::Comma => "Comma",
        TokenKind::Arrow => "Arrow",
        TokenKind::Less => "Less",
        TokenKind::LessOrEqual => "LessOrEqual",
        TokenKind::Greater => "Greater",
        TokenKind::GreaterOrEqual => "GreaterOrEqual",
        TokenKind::Or => "Or",
        TokenKind::And => "And",
        TokenKind::Else => "Else",
        TokenKind::Colon => "Colon",
        TokenKind::Load => "Load",
        TokenKind::Extern => "Extern",
        TokenKind::Fn => "Fn",
        TokenKind::End => "End",
        TokenKind::Struct => "Struct",
        TokenKind::Dot => "Dot",
        TokenKind::DoubleColon => "DoubleColon",
        TokenKind::Pipe => "Pipe",
        TokenKind::Return => "Return"
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TokenKind::Numeric(value) => {
                write!(f, "<{}({})>", token_name(self), value)
            }
            TokenKind::Identifier(literal) | TokenKind::String(literal) => {
                write!(f, "<{}({})>", token_name(self), literal)
            }
            _ => write!(f, "<{}>", token_name(self)),
        }
    }
}
