use std::{fmt, str, iter};


pub struct Token {
    pub position: usize,
    pub details: TokenKind
}

pub enum TokenKind {
    Paren(Paren), Op(Op),
    Keyword(Keyword), Name(String),
    Literal(Literal)
}

pub enum Paren { Left, Right }

pub enum Op {
    Plus, Minus, Times, DivBy,
    Less, LessEq, GreaterEq, Greater,
    Equals, NotEquals,
    Not, And, Or,
    Assign
}

pub enum Keyword { If, Then, Else }

pub enum Literal { Int(i32), Float(f64), Bool(bool) }


pub struct TokenIter<I: iter::Iterator<Item = char>> {
    input: iter::Peekable<iter::Enumerate<I>>
}

impl<I: iter::Iterator<Item = char>> TokenIter<I> {
    pub fn new(chars: I) -> TokenIter<I> {
        TokenIter { input: chars.enumerate().peekable() }
    }
}

enum ParsingState {
    Entry,
    Whitespace,
    Less, Greater, Equal, Not, Ampersand, Bar, Colon,
//    Int, Float,
//    I,
//    T, Th, The,
//    E, El, Els,
//    Tr, Tru,
//    F, Fa, Fal, Fals
}

enum TokenState {
    Building(String, ParsingState),
    Finished(TokenKind)
}

fn read_char(token: &mut TokenState, c: char) -> bool {
    let (memorized, state) = match token {
        TokenState::Finished(_) => return false,
        TokenState::Building(memorized, state) => (memorized, state)
    };
    match state {
        ParsingState::Entry | ParsingState::Whitespace => match c {
            ' ' | '\n' => {
                *state = ParsingState::Whitespace;
                true
            },
            '(' => {
                *token = TokenState::Finished(TokenKind::Paren(Paren::Left));
                true
            },
            ')' => {
                *token = TokenState::Finished(TokenKind::Paren(Paren::Right));
                true
            },
            '+' => {
                *token = TokenState::Finished(TokenKind::Op(Op::Plus));
                true
            },
            '-' => {
                *token = TokenState::Finished(TokenKind::Op(Op::Minus));
                true
            },
            '*' => {
                *token = TokenState::Finished(TokenKind::Op(Op::Times));
                true
            },
            '/' => {
                *token = TokenState::Finished(TokenKind::Op(Op::DivBy));
                true
            },
            '<' => {
                *state = ParsingState::Less;
                true
            },
            '>' => {
                *state = ParsingState::Greater;
                true
            },
            '=' => {
                *state = ParsingState::Equal;
                true
            },
            '!' => {
                *state = ParsingState::Not;
                true
            },
            '&' => {
                *state = ParsingState::Ampersand;
                true
            },
            '|' => {
                *state = ParsingState::Bar;
                true
            },
            ':' => {
                *state = ParsingState::Colon;
                true
            },
            _ => {
                *state = ParsingState::Entry;
                false
            }
        },
        ParsingState::Less => match c {
            '=' => {
                *token = TokenState::Finished(TokenKind::Op(Op::LessEq));
                true
            },
            _ => {
                *token = TokenState::Finished(TokenKind::Op(Op::Less));
                false
            }
        },
        ParsingState::Greater => match c {
            '=' => {
                *token = TokenState::Finished(TokenKind::Op(Op::GreaterEq));
                true
            },
            _ => {
                *token = TokenState::Finished(TokenKind::Op(Op::Greater));
                false
            }
        },
        ParsingState::Equal => match c {
            '=' => {
                *token = TokenState::Finished(TokenKind::Op(Op::Equals));
                true
            },
            _ => false
        },
        ParsingState::Not => match c {
            '=' => {
                *token = TokenState::Finished(TokenKind::Op(Op::NotEquals));
                true
            },
            _ => {
                *token = TokenState::Finished(TokenKind::Op(Op::Not));
                false
            }
        },
        ParsingState::Ampersand => match c {
            '&' => {
                *token = TokenState::Finished(TokenKind::Op(Op::And));
                true
            },
            _ => false
        },
        ParsingState::Bar => match c {
            '|' => {
                *token = TokenState::Finished(TokenKind::Op(Op::Or));
                true
            },
            _ => false
        },
        ParsingState::Colon => match c {
            '=' => {
                *token = TokenState::Finished(TokenKind::Op(Op::Assign));
                true
            },
            _ => false
        }
    }
}

impl<I: iter::Iterator<Item = char>> iter::Iterator for TokenIter<I> {
    type Item = Result<Token, (Option<usize>, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut position, _) = *self.input.peek()?;

        let mut current_state = TokenState::Building(
            String::new(), ParsingState::Entry
        );
        while let Some(_) = self.input.next_if(|elt| {
            let (pos, character) = *elt;
            let ret = read_char(&mut current_state, character);
            if
                let TokenState::Building(_, ParsingState::Whitespace) =
                current_state
            {
                position = pos + 1;
            }
            ret
        }) {};

        match current_state {
            TokenState::Building(_, ParsingState::Whitespace) => None,
            TokenState::Building(..) => Some(Err(match self.input.peek() {
                None => (
                    None, "Unexpected end of text while tokenizing.".to_string()
                ),
                Some((pos, c)) => (
                    Some(*pos), format!("Unexpected character '{}'", c)
                )
            })),
            TokenState::Finished(details) => Some(
                Ok(Token { position, details })
            )
        }
    }
}


impl<I: iter::Iterator<Item = char> + Clone> Clone for TokenIter<I> {
    fn clone(&self) -> TokenIter<I> {
        TokenIter { input: self.input.clone() }
    }
}

impl<I: iter::Iterator<Item = char> + Clone> fmt::Debug for TokenIter<I> {
    // Warning: printing a TokenIter will make a copy of the TokenIter, scan the
    // input, and won't cache the results. If you then iterate through your
    // original TokenIter, it will do the same work twice.
    // Since TokenIter runs in linear time, and this function is for debugging,
    // it shouldn't be a problem in most cases.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first_time = true;
        let mut decide_space = |space| if first_time {
            first_time = false;
            ""
        } else {
            space
        };
        for token in self.clone() {
            match token {
                Ok(token) =>
                    write!(f, "{}{:?}", decide_space(" "), token)?,
                Err((position, message)) => {
                    write!(f, "{}", decide_space("\n"))?;
                    if let Some(position) = position {
                        write!(f, "/!\\ At position {}:\n", position)?
                    }
                    return write!(f, "/!\\ {}", message);
                }
            };
        }
        fmt::Result::Ok(())
    }
}


impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}:{:?}", self.position, self.details)
    }
}

impl fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Paren(paren) => write!(f, "{:?}", paren),
            Self::Op(binop) => write!(f, "{:?}", binop),
            Self::Keyword(keyword) => write!(f, "{:?}", keyword),
            Self::Name(name) => write!(f, "Name({})", name),
            Self::Literal(literal) => write!(f, "{:?}", literal)
        }
    }
}

impl fmt::Debug for Paren {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => write!(f, "LeftParen"),
            Self::Right => write!(f, "RightParen")
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Times => write!(f, "Times"),
            Self::DivBy => write!(f, "DivBy"),
            Self::Less => write!(f, "Less"),
            Self::LessEq => write!(f, "LessEq"),
            Self::GreaterEq => write!(f, "GreaterEq"),
            Self::Greater => write!(f, "Greater"),
            Self::Equals => write!(f, "Equals"),
            Self::NotEquals => write!(f, "NotEquals"),
            Self::Not => write!(f, "Not"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Assign => write!(f, "Assign")
        }
    }
}

impl fmt::Debug for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::If => write!(f, "If"),
            Self::Then => write!(f, "Then"),
            Self::Else => write!(f, "Else")
        }
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(x) => write!(f, "Int({})", x),
            Self::Float(x) => write!(f, "Float({})", x),
            Self::Bool(x) => write!(f, "Bool({})", x)
        }
    }
}
