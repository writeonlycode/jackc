use anyhow::{bail, Result};
use std::{fmt::Debug, fs::File, io::BufReader, iter::Peekable};
use utf8_chars::{BufReadCharsExt, Chars};

pub struct Tokenizer<'a> {
    file: Peekable<Chars<'a, BufReader<File>>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(file: &mut BufReader<File>) -> Tokenizer {
        Tokenizer {
            file: file.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        let chars = &mut self.file;

        loop {
            let current_char = match chars.next() {
                Some(Ok(value)) => value,
                _ => return None,
            };

            buffer.push(current_char);

            match buffer.as_str() {
                "" => {}
                "/" => {
                    let peek = match chars.peek() {
                        Some(Ok(value)) => value,
                        _ => &'\n',
                    };

                    if peek == &'/' {
                        while chars.next().unwrap().unwrap() != '\n' {}
                        buffer.clear();
                    } else if peek == &'*' {
                        let mut temporary_buffer = String::new();

                        while !temporary_buffer.ends_with("*/") {
                            match chars.next() {
                                Some(Ok(value)) => temporary_buffer.push(value.clone()),
                                _ => break,
                            }
                        }

                        buffer.clear();
                    } else {
                        buffer.clear();
                        return Some(Token::Symbol(Symbol::Divide));
                    }
                }
                " " => {
                    buffer.clear();
                }
                "\t" => {
                    buffer.clear();
                }
                "\n" => {
                    buffer.clear();
                }
                "\r\n" => {
                    buffer.clear();
                }
                "(" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::LeftRoundBracket));
                }
                ")" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::RightRoundBracket));
                }
                "[" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::LeftSquareBracket));
                }
                "]" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::RightSquareBracket));
                }
                "{" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::LeftCurlyBracket));
                }
                "}" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::RightCurlyBracket));
                }
                "," => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Comma));
                }
                ";" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Semicolon));
                }
                "=" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Equal));
                }
                "." => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Dot));
                }
                "+" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Plus));
                }
                "-" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Minus));
                }
                "*" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Times));
                }
                "&" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::And));
                }
                "|" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Or));
                }
                "~" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::Not));
                }
                "<" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::SmallerThan));
                }
                ">" => {
                    buffer.clear();
                    return Some(Token::Symbol(Symbol::GreaterThan));
                }
                "class" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Class));
                }
                "constructor" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Constructor));
                }
                "method" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Method));
                }
                "function" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Function));
                }
                "int" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Int));
                }
                "boolean" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Boolean));
                }
                "char" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Char));
                }
                "void" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Void));
                }
                "var" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Var));
                }
                "static" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Static));
                }
                "field" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Field));
                }
                "let" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Let));
                }
                "do" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Do));
                }
                "if" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::If));
                }
                "else" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Else));
                }
                "while" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::While));
                }
                "return" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Return));
                }
                "true" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::True));
                }
                "false" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::False));
                }
                "null" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::Null));
                }
                "this" => {
                    buffer.clear();
                    return Some(Token::Keyword(Keyword::This));
                }
                "\"" => {
                    buffer.clear();

                    while !buffer.ends_with("\"") {
                        buffer.push(chars.next().unwrap().unwrap());
                    }

                    let buffer = buffer.trim_end_matches("\"");
                    return Some(Token::StringConstant(buffer.to_string()));
                }
                _ => {
                    let peek = match chars.peek() {
                        Some(Ok(value)) => value,
                        _ => &'\n',
                    };

                    if buffer.chars().next().unwrap().is_ascii_alphabetic()
                        && !peek.is_ascii_alphabetic()
                        && !peek.is_ascii_digit()
                    {
                        let identifier_value = IdentifierValue::new(buffer.clone()).unwrap();
                        buffer.clear();
                        return Some(Token::Identifier(identifier_value));
                    }

                    if buffer.chars().next().unwrap().is_ascii_digit() && !peek.is_ascii_digit() {
                        let integer_constant_value =
                            IntegerConstantValue::new(buffer.parse().unwrap()).unwrap();
                        buffer.clear();
                        return Some(Token::IntegerConstant(integer_constant_value));
                    }
                }
            }
        }
    }
}

#[derive(PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(IntegerConstantValue),
    StringConstant(String),
    Identifier(IdentifierValue),
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "<keyword> {:?} </keyword>", keyword),
            Token::Symbol(symbol) => write!(f, "<symbol> {:?} </symbol>", symbol),
            Token::IntegerConstant(integer_constant) => {
                write!(
                    f,
                    "<integerConstant> {} </integerConstant>",
                    integer_constant.value
                )
            }
            Token::StringConstant(string_constant) => {
                write!(f, "<stringConstant> {} </stringConstant>", string_constant)
            }
            Token::Identifier(identifier_value) => {
                write!(f, "<identifier> {} </identifier>", identifier_value.value)
            }
        }
    }
}

#[derive(PartialEq)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

impl Debug for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Class => write!(f, "class"),
            Keyword::Constructor => write!(f, "constructor"),
            Keyword::Function => write!(f, "function"),
            Keyword::Method => write!(f, "method"),
            Keyword::Field => write!(f, "field"),
            Keyword::Static => write!(f, "static"),
            Keyword::Var => write!(f, "var"),
            Keyword::Int => write!(f, "int"),
            Keyword::Char => write!(f, "char"),
            Keyword::Boolean => write!(f, "boolean"),
            Keyword::Void => write!(f, "void"),
            Keyword::True => write!(f, "true"),
            Keyword::False => write!(f, "false"),
            Keyword::Null => write!(f, "null"),
            Keyword::This => write!(f, "this"),
            Keyword::Let => write!(f, "let"),
            Keyword::Do => write!(f, "do"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::While => write!(f, "while"),
            Keyword::Return => write!(f, "return"),
        }
    }
}

#[derive(PartialEq)]
pub enum Symbol {
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftRoundBracket,
    RightRoundBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Times,
    Divide,
    And,
    Or,
    SmallerThan,
    GreaterThan,
    Equal,
    Not,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::LeftCurlyBracket => write!(f, "{{"),
            Symbol::RightCurlyBracket => write!(f, "}}"),
            Symbol::LeftRoundBracket => write!(f, "("),
            Symbol::RightRoundBracket => write!(f, ")"),
            Symbol::LeftSquareBracket => write!(f, "["),
            Symbol::RightSquareBracket => write!(f, "]"),
            Symbol::Dot => write!(f, "."),
            Symbol::Comma => write!(f, ","),
            Symbol::Semicolon => write!(f, ";"),
            Symbol::Plus => write!(f, "+"),
            Symbol::Minus => write!(f, "-"),
            Symbol::Times => write!(f, "*"),
            Symbol::Divide => write!(f, "/"),
            Symbol::And => write!(f, "&amp;"),
            Symbol::Or => write!(f, "|"),
            Symbol::SmallerThan => write!(f, "&lt;"),
            Symbol::GreaterThan => write!(f, "&gt;"),
            Symbol::Equal => write!(f, "="),
            Symbol::Not => write!(f, "~"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IntegerConstantValue {
    pub value: u16,
}

impl IntegerConstantValue {
    pub fn new(value: u16) -> Result<IntegerConstantValue> {
        if value <= 32767 {
            return Ok(IntegerConstantValue { value });
        } else {
            bail!("an IntegerConstant must have a value in the range 0...32767")
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IdentifierValue {
    pub value: String,
}

impl IdentifierValue {
    pub fn new(value: String) -> Result<IdentifierValue> {
        if !value.chars().next().unwrap().is_numeric() {
            return Ok(IdentifierValue { value });
        } else {
            bail!("an Identifier can't start with a digit")
        }
    }
}
