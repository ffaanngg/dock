#![allow(dead_code)]

pub(crate) enum Token {
    Program(String),
    Command(String),
    Arg(String),
}

pub(crate) struct Parser {
    raw: String,
    tokens: Vec<Token>,
}
