mod lexer;
mod token;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_test() {
        println!("{:?}", lexer::lex("( A NAND B ) XOR ( A NOR C ) = Q"));
    }
}
