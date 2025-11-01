#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Mul,
    Do,
    Dont,
    Num(u32),
    LeftParens,
    RightParens,
    Separator,
    Other,
}

pub struct Lexer<'s> {
    source: &'s str,
    read_position: usize,
}

impl Lexer<'_> {
    #[must_use]
    pub const fn new(source: &str) -> Lexer {
        Lexer {
            source,
            read_position: 0,
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.source.chars().nth(self.read_position + offset)
    }

    const fn advance(&mut self, offset: usize) {
        self.read_position += offset;
    }

    fn match_word(&self, word: &str) -> bool {
        self.source
            .chars()
            .skip(self.read_position)
            .zip(word.chars())
            .all(|(a, b)| a == b)
    }

    fn read_number(&self) -> u32 {
        self.source
            .chars()
            .skip(self.read_position)
            .take_while(char::is_ascii_digit)
            .fold(0, |acc, x| acc * 10 + x.to_digit(10).unwrap())
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek(0) {
            Some('m') if self.match_word("mul") => {
                self.advance(3);
                Some(Token::Mul)
            }
            Some('d') if self.match_word("don't") => {
                self.advance(5);
                Some(Token::Dont)
            }
            Some('d') if self.match_word("do") => {
                self.advance(2);
                Some(Token::Do)
            }
            Some('0'..='9') => {
                let value = self.read_number();
                self.advance(if value == 0 {
                    1
                } else {
                    value.ilog10() as usize + 1
                });
                Some(Token::Num(value))
            }
            Some('(') => {
                self.advance(1);
                Some(Token::LeftParens)
            }
            Some(')') => {
                self.advance(1);
                Some(Token::RightParens)
            }
            Some(',') => {
                self.advance(1);
                Some(Token::Separator)
            }
            Some(_) => {
                self.advance(1);
                Some(Token::Other)
            }
            None => None,
        }
    }
}
