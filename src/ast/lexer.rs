#[derive(Debug)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    EOF,
    Bad,
}

#[derive(Debug)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos > self.input.len() {
            return None;
        }

        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, "\0".to_string()),
            ));
        }

        let start = self.current_pos;
        let c = self.current_char();
        let mut token = TokenKind::Bad;
        if Self::is_number_start(&c) {
            let number: i64 = self.consume_number();
            token = TokenKind::Number(number)
        }

        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);
        Some(Token::new(token, span))
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.consume() {
            if c.is_digit(10) {
                number = number * 10 + c.to_digit(10).unwrap() as i64
            } else {
                break;
            }
        }
        number
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        Some(c)
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap()
    }
}
