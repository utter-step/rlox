use phf::phf_map;

#[derive(Debug, Clone, Copy)]
pub enum TokenValue<'a> {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(&'a str),
    String(&'a str),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub static KEYWORDS: phf::Map<&'static str, TokenValue> = phf_map! {
    "and" => TokenValue::And,
    "class" => TokenValue::Class,
    "else" => TokenValue::Else,
    "false" => TokenValue::False,
    "fun" => TokenValue::Fun,
    "for" => TokenValue::For,
    "if" => TokenValue::If,
    "nil" => TokenValue::Nil,
    "or" => TokenValue::Or,
    "print" => TokenValue::Print,
    "return" => TokenValue::Return,
    "super" => TokenValue::Super,
    "this" => TokenValue::This,
    "true" => TokenValue::True,
    "var" => TokenValue::Var,
    "while" => TokenValue::While,
};

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    value: TokenValue<'a>,
    lexeme: &'a str,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(value: TokenValue<'a>, lexeme: &'a str, line: usize) -> Self {
        Self {
            value,
            lexeme,
            line,
        }
    }
}
