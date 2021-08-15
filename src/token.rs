use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Letter(char),
    And,
    Or,
    Xor,
    Not,
    Nand,
    Nor,
    Xnor,
    Equals,
}

impl FromStr for Token {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Token::*;
        Ok(match s {
            "(" => OpenParen,
            ")" => CloseParen,
            "and" | "&" | "&&" => And,
            "or" | "|" | "||" => Or,
            "xor" => Xor,
            "not" | "!" => Not,
            "nand" => Nand,
            "nor" => Nor,
            "xnor" => Xnor,
            "equals" | "=" => Equals,
            _ => {
                if s.len() == 1 && s.is_ascii() {
                    Letter(s.chars().next().unwrap())
                } else {
                    unreachable!()
                }
            }
        })
    }
}
