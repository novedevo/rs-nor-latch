use std::str::FromStr;

use crate::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    input
        .to_lowercase()
        .split_whitespace()
        .map(|s| Token::from_str(s).unwrap())
        .collect()
}
