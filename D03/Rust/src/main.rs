#[derive(Debug)]
enum Token {
    Mul,
    Do,
    Dont,
    Num(u32),
    LeftParens,
    RightParens,
    Separator,
    Other(char),
    End,
}

struct Lexer<'s> {
    source: &'s str,
    read_position: usize,
}

impl<'s> Lexer<'s> {
    fn new(source: &str) -> Lexer {
        Lexer {
            source,
            read_position: 0,
        }
    }

    fn next(&mut self) -> Token {
        match self.source.chars().nth(self.read_position) {
            Some('m') => {
                if &self.source[self.read_position..self.read_position + 3] == "mul" {
                    self.read_position += 3;
                    Token::Mul
                } else {
                    self.read_position += 1;
                    Token::Other('m')
                }
            }
            Some('d') => {
                if &self.source[self.read_position..self.read_position + 5] == "don't" {
                    self.read_position += 5;
                    Token::Dont
                } else if &self.source[self.read_position..self.read_position + 2] == "do" {
                    self.read_position += 2;
                    Token::Do
                } else {
                    self.read_position += 1;
                    Token::Other('d')
                }
            }
            Some('0'..='9') => {
                let (tok, offset) = Self::read_number(&self.source[self.read_position..]);
                if let Some(tok) = tok {
                    self.read_position += offset;
                    tok
                } else {
                    Token::End
                }
            }
            Some('(') => {
                self.read_position += 1;
                Token::LeftParens
            }
            Some(')') => {
                self.read_position += 1;
                Token::RightParens
            }
            Some(',') => {
                self.read_position += 1;
                Token::Separator
            }
            Some(c) => {
                self.read_position += 1;
                Token::Other(c)
            }
            None => Token::End,
        }
    }

    fn read_number(input: &str) -> (Option<Token>, usize) {
        let mut num_str = String::new();
        let mut offset = 0;
        while offset < 4 {
            if let Some(d @ '0'..='9') = input.chars().nth(offset) {
                num_str.push(d);
            } else {
                break;
            }
            offset += 1;
        }
        let value = num_str.parse::<u32>().unwrap();
        (Some(Token::Num(value)), offset)
    }
}

#[derive(Debug)]
struct Multiply {
    first: u32,
    second: u32,
}

struct Parser<'s, 'l> {
    lexer: &'l mut Lexer<'s>,
    enable_multiplication: bool,
    enable_conditionals: bool,
}

impl<'s, 'l: 's> Parser<'s, 'l> {
    fn new(lexer: &'l mut Lexer<'s>, conditions_enabled: bool) -> Parser<'s, 'l> {
        Parser {
            lexer,
            enable_conditionals: conditions_enabled,
            enable_multiplication: true,
        }
    }

    fn parse(&mut self) -> Option<Multiply> {
        match self.lexer.next() {
            Token::Mul => {
                let mul = self.parse_mul();

                if let Some(mul) = mul {
                    if self.enable_multiplication {
                        Some(mul)
                    } else {
                        Some(Multiply {
                            first: 0,
                            second: 0,
                        })
                    }
                } else {
                    Some(Multiply {
                        first: 0,
                        second: 0,
                    })
                }
            }
            Token::Do if self.enable_conditionals => {
                self.enable_multiplication = true;
                Some(Multiply {
                    first: 0,
                    second: 0,
                })
            }
            Token::Dont if self.enable_conditionals => {
                self.enable_multiplication = false;
                Some(Multiply {
                    first: 0,
                    second: 0,
                })
            }
            Token::End => None,
            _ => Some(Multiply {
                first: 0,
                second: 0,
            }),
        }
    }

    fn parse_mul(&mut self) -> Option<Multiply> {
        if !matches!(self.lexer.next(), Token::LeftParens) {
            return None;
        }

        let Token::Num(first) = self.lexer.next() else {
            return None;
        };

        if !matches!(self.lexer.next(), Token::Separator) {
            return None;
        }

        let Token::Num(second) = self.lexer.next() else {
            return None;
        };

        if !matches!(self.lexer.next(), Token::RightParens) {
            return None;
        }

        Some(Multiply { first, second })
    }
}

fn solve_part_one(input: &str) -> u32 {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer, false);

    let mut total = 0;

    while let Some(Multiply { first, second }) = parser.parse() {
        total += first * second;
    }

    total
}

fn solve_part_two(input: &str) -> u32 {
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer, true);

    let mut total = 0;

    while let Some(Multiply { first, second }) = parser.parse() {
        total += first * second;
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("../input.txt")?;

    let p1 = solve_part_one(&data);

    println!("{p1}");

    let p2 = solve_part_two(&data);

    println!("{p2}");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sample_one() {
        const SAMPLE: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected = 161;
        let actual = super::solve_part_one(SAMPLE);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sample_two() {
        const SAMPLE: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let expected = 48;
        let actual = super::solve_part_two(SAMPLE);
        assert_eq!(expected, actual);
    }
}
