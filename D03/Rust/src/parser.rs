use crate::lexer::{Lexer, Token};

pub enum Expression {
    Multiply(u32, u32),
    Nop,
}

pub struct Parser<'s, 'l> {
    lexer: &'l mut Lexer<'s>,
    enable_multiplication: bool,
    enable_conditionals: bool,
}

impl<'s, 'l: 's> Parser<'s, 'l> {
    pub const fn new(lexer: &'l mut Lexer<'s>, conditions_enabled: bool) -> Self {
        Parser {
            lexer,
            enable_conditionals: conditions_enabled,
            enable_multiplication: true,
        }
    }

    fn parse_mul(&mut self) -> Option<Expression> {
        if self.lexer.next() != Some(Token::LeftParens) {
            return None;
        }

        let Some(Token::Num(first)) = self.lexer.next() else {
            return None;
        };

        if self.lexer.next() != Some(Token::Separator) {
            return None;
        }

        let Some(Token::Num(second)) = self.lexer.next() else {
            return None;
        };

        if self.lexer.next() != Some(Token::RightParens) {
            return None;
        }

        Some(Expression::Multiply(first, second))
    }
}

impl<'s, 'l: 's> Iterator for Parser<'s, 'l> {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(Token::Mul) => match self.parse_mul() {
                Some(mul) if self.enable_multiplication => Some(mul),
                Some(_) | None => Some(Expression::Nop),
            },
            Some(Token::Do) if self.enable_conditionals => {
                self.enable_multiplication = true;
                Some(Expression::Nop)
            }
            Some(Token::Dont) if self.enable_conditionals => {
                self.enable_multiplication = false;
                Some(Expression::Nop)
            }
            Some(_) => Some(Expression::Nop),
            None => None,
        }
    }
}
