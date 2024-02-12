#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: Option<String>
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.text {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{:?}", self.kind)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    QuotedString,
    Number,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Keyword
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable()
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(&c) = self.chars.peek() {
            return if c == '"' {
                self.chars.next();
                let text = self.consume_while(|c| c != '"');

                self.chars.next();
                Some(Token {
                    kind: TokenKind::QuotedString,
                    text: Some(text)
                })
            } else if c.is_numeric() {
                Some(Token {
                    kind: TokenKind::Number,
                    text: Some(self.consume_while(|c| c.is_numeric()))
                })
            } else if c == '(' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::OpenParen,
                    text: None
                })
            } else if c == ')' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::CloseParen,
                    text: None
                })
            } else if c == '[' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::OpenBracket,
                    text: None
                })
            } else if c == ']' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::CloseBracket,
                    text: None
                })
            } else if c == '{' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::OpenBrace,
                    text: None
                })
            } else if c == '}' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::CloseBrace,
                    text: None
                })
            } else if c == ':' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::Colon,
                    text: None
                })
            } else if c == ',' {
                self.chars.next();
                Some(Token {
                    kind: TokenKind::Comma,
                    text: None
                })
            } else if c.is_whitespace() {
                self.chars.next();
                continue;
            } else {
                return Some(Token {
                    kind: TokenKind::Keyword,
                    text: Some(self.consume_while(|c| c.is_alphabetic()))
                })
            };
        }

        None
    }

    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut result = String::new();

        while let Some(&c) = self.chars.peek() {
            if predicate(c) {
                result.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        result
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        Ok(tokens)
    }
}