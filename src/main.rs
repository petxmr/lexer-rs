#[warn(dead_code)]
use regex::Regex;

#[derive(Debug)]
enum Token {
    Identifier(String),
    Number(f64),
    Operator(String),
    Parenthesis(char),
    Assignment,
}

fn lexer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let identifier = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let number = Regex::new(r"^\d+(\.\d+)?").unwrap();
    let operator = Regex::new(r"^[+\-*/]").unwrap();
    let parenthesis = Regex::new(r"^[(){}\[\]]").unwrap();
    let assignment = Regex::new(r"^=").unwrap();

    let mut remaining: &str = input.trim();

    while !remaining.is_empty() {
        if let Some(captures) = identifier.captures(remaining) {
            let id = captures[0].to_string();
            tokens.push(Token::Identifier(id.clone()));
            remaining = &remaining[id.len()..];
        } else if let Some(captures) = number.captures(remaining) {
            let num: f64 = captures[0].parse::<f64>().unwrap();
            tokens.push(Token::Number(num));
            remaining = &remaining[captures[0].len()..];
        } else if let Some(captures) = operator.captures(remaining) {
            let op = captures[0].to_string();
            tokens.push(Token::Operator(op.clone()));
            remaining = &remaining[op.len()..];
        } else if let Some(captures) = parenthesis.captures(remaining) {
            let parens: char = captures[0].chars().next().unwrap();
            tokens.push(Token::Parenthesis(parens));
            remaining = &remaining[captures[0].len()..];
        } else if let Some(captures) = assignment.captures(remaining) {
            tokens.push(Token::Assignment);
            remaining = &remaining[captures[0].len()..];
        } else {
            panic!("Unexpected char: {}", remaining.chars().next().unwrap());
        }

        remaining = remaining.trim();
    }

    tokens
}

fn main() {
    let input: &str = "x = 10 + 5 * (3 - 2)";
    let tokens: Vec<Token> = lexer(input);

    println!("Tokens: {:?}", tokens);
}
