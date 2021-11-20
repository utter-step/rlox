use std::cell::Cell;

use crate::{
    error,
    token::{Token, TokenValue, KEYWORDS},
};

macro_rules! _token {
    ($self:ident, $type:ident) => {{
        // safe to unwrap, we've checked previously, that it's not at chars boundary
        Some(Token::new(
            TokenValue::$type,
            $self.cur_slice().unwrap(),
            $self.line.get(),
        ))
    }};
    ($self:ident, $type:ident, $value:expr) => {{
        // safe to unwrap, we've checked previously, that it's not at chars boundary
        Some(Token::new(
            TokenValue::$type($value),
            $self.cur_slice().unwrap(),
            $self.line.get(),
        ))
    }};
    ($self:ident, $value:expr) => {{
        // safe to unwrap, we've checked previously, that it's not at chars boundary
        Some(Token::new(
            $value,
            $self.cur_slice().unwrap(),
            $self.line.get(),
        ))
    }};
}

#[derive(Debug, Default)]
pub struct Scanner<'a> {
    source: &'a str,
    start: Cell<usize>,
    current: Cell<usize>,
    line: Cell<usize>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            line: Cell::new(1),
            ..Default::default()
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        self.reset();

        tokens
    }

    fn reset(&self) {
        self.start.set(0);
        self.current.set(0);

        self.line.set(0);
    }

    fn scan_token(&self) -> Option<Token> {
        macro_rules! token {
            ($type:ident) => {
                _token!(self, $type)
            };
        }

        self.start.set(self.current.get());

        let c = if let Some(c) = self.advance() {
            c
        } else {
            return Some(Token::new(
                crate::token::TokenValue::Eof,
                "",
                self.line.get(),
            ));
        };

        match c {
            '(' => token!(LeftParen),
            ')' => token!(RightParen),
            '{' => token!(LeftBrace),
            '}' => token!(RightBrace),
            ',' => token!(Comma),
            '.' => token!(Dot),
            '-' => token!(Minus),
            '+' => token!(Plus),
            ';' => token!(Semicolon),
            '*' => token!(Star),
            '!' => {
                if self.match_next("=") {
                    token!(BangEqual)
                } else {
                    token!(Bang)
                }
            }
            '=' => {
                if self.match_next("=") {
                    token!(EqualEqual)
                } else {
                    token!(Equal)
                }
            }
            '>' => {
                if self.match_next("=") {
                    token!(GreaterEqual)
                } else {
                    token!(Greater)
                }
            }
            '<' => {
                if self.match_next("=") {
                    token!(LessEqual)
                } else {
                    token!(Less)
                }
            }
            '/' => {
                if self.match_next("/") {
                    loop {
                        match self.peek() {
                            Some(c) if c != '\n' => {
                                self.advance();
                                self.start.set(self.start.get() + c.len_utf8());
                            }
                            _ => break,
                        }
                    }

                    // advance over newline
                    // self.start.set(self.start.get() + 1);

                    None
                } else {
                    token!(Slash)
                }
            }
            ' ' | '\t' | '\r' => None,
            '\n' => {
                self.line.set(self.line.get() + 1);

                None
            }
            '\"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_alphanumeric() => self.identifier(),
            _ => {
                error(self.line.get(), "unexpected character");

                None
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current.get() >= self.source.len()
    }

    fn cur_slice(&self) -> Option<&str> {
        self.source.get(self.start.get()..self.current.get())
    }

    fn advance(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let previous = self.current.get();
        let mut current = previous + 1;

        while !self.source.is_char_boundary(current) {
            current += 1;
        }

        self.current.set(current);

        self.source.get(previous..current).unwrap().chars().next()
    }

    /// advance over next symbol if it matches
    fn match_next(&self, next: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current = self.current.get();
        let next_current = current + next.len();

        if self.source.get(current..next_current) != Some(next) {
            return false;
        }

        self.current.set(next_current);
        true
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let mut next = self.current.get() + 1;

        while !self.source.is_char_boundary(next) {
            next += 1;
        }

        self.source
            .get(self.current.get()..next)
            .unwrap()
            .chars()
            .next()
    }

    fn peek_next(&self) -> Option<char> {
        let current_c = self.peek()?;
        let next_start = self.current.get() + current_c.len_utf8();
        let mut next_end = next_start + 1;

        while !self.source.is_char_boundary(next_end) {
            next_end += 1;
        }

        self.source
            .get(next_start..next_end)
            .unwrap()
            .chars()
            .next()
    }

    fn identifier(&self) -> Option<Token> {
        macro_rules! token {
            ($type:ident, $value:expr) => {
                _token!(self, $type, $value)
            };
            ($value:expr) => {
                _token!(self, $value)
            };
        }

        while matches!(self.peek(), Some(c) if c.is_alphanumeric()) {
            self.advance();
        }

        if let Some(token) = KEYWORDS.get(self.cur_slice().unwrap()) {
            token!(*token)
        } else {
            token!(Identifier, self.cur_slice().unwrap())
        }
    }

    fn number(&self) -> Option<Token> {
        macro_rules! token {
            ($type:ident, $value:expr) => {
                _token!(self, $type, $value)
            };
        }

        while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            self.advance();
        }

        if self.peek() == Some('.') && matches!(self.peek_next(), Some(c) if c.is_ascii_digit()) {
            // advance the dot and following digit
            self.advance();
            self.advance();

            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.advance();
            }
        }

        // safe to unwrap here, as we've just advanced
        let number = self.cur_slice().unwrap().parse();

        match number {
            Ok(num) => token!(Number, num),
            Err(e) => {
                error(self.line.get(), e.to_string());

                None
            }
        }
    }

    fn string(&self) -> Option<Token> {
        macro_rules! token {
            ($type:ident, $value:expr) => {
                _token!(self, $type, $value)
            };
        }

        loop {
            let next = self.peek();

            match next {
                Some('\n') => {
                    self.line.set(self.line.get() + 1);

                    self.advance();
                }
                Some('\"') => {
                    // skip last "
                    self.advance();

                    break;
                }
                Some(_) => {
                    self.advance();
                }
                None => {
                    error(self.line.get(), "Unterminated string.");

                    return None;
                }
            }
        }

        // safe to unwrap, as advance checks char boundaries
        let string_value = &self.source[self.start.get() + 1..self.current.get() - 1];
        token!(String, string_value)
    }
}
