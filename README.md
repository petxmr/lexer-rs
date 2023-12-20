# lexer-rs

### another lexer but in rust

### usage

```rust
fn main() {
    let input: &str = "x = 10 + 5 * (3 - 2)";
    let tokens: Vec<Token> = lexer(input);

    println!("Tokens: {:?}", tokens);
}
```

### output
```rust
Tokens: [Identifier("x"), Assignment, Number(10.0), Operator("+"), Number(5.0), Operator("*"), Parenthesis('('), Number(3.0), Operator("-"), Number(2.0), Parenthesis(')')]
```
