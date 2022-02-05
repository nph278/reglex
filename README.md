# reglex
A small rust library for creating regex-based lexers

## Example

```rust

use reglex::{RuleList, rule_list, lex};

#[derive(Debug, PartialEq)]
enum Token {
    Keyword,
    Number(u64),
    Left,
    Right,
}

fn lexer(input: &String) -> Result<Vec<Token>, usize> {
    let regexes: RuleList<Token> = rule_list! [
        "kw" => |_| Some(Token::Keyword),
        r"\d+" => |s: &str| Some(Token::Number(s.parse().unwrap())),
        r"\{" => |_| Some(Token::Left),
        r"\}" => |_| Some(Token::Right),
        r"\s" => |_| None
    ];

    lex(&regexes, input)
}

fn main() {
    assert_eq!(
        lexer(&"kw  { 12 53 }".to_string()),
        Ok(vec![
            Token::Keyword,
            Token::Left,
            Token::Number(12),
            Token::Number(53),
            Token::Right
        ])
    );

    assert_eq!(lexer(&"kw ERROR! { 12 53 }".to_string()), Err(3));
}
```

