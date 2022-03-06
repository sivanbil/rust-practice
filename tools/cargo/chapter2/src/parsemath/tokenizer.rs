use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let next_chat = self.expr.next();

        match next_char {
            Some('0'...='9') => {
                let mut number = next_chat?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_chat.is_numberic() || next_chat == &'.' {
                        number.push(self.expr.next()?);
                    } else if next_char == &'('{
                        return None;
                    } else {
                        break;
                    }
                }

                Some(Token::Num(number.parse::<f64>().unwrap()))
            },

            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Substract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}