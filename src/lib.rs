#[macro_export]
macro_rules! reglex {
    ($type:ty, $($pat:expr => $fn:expr), *) => {
        {
            let regexes: Vec<(Regex, Box<dyn Fn(&str) -> Option<$type>>)> = vec![
                $((Regex::new($pat), Box::new($fn)))*
            ];

            move |s: String| {
                let mut pos = 0;
                let mut output: Vec<Token> = Vec::new();

                while pos < s.len() {
                    let mut bad = true;

                    for (pat, f) in &regexes {
                        if let Some(m) = pat.find_at(&s, pos) {
                            if m.start() == pos {
                                bad = false;

                                match f(m.as_str()) {
                                    Some(t) => {
                                        output.push(t);
                                        pos = m.end();
                                        break;
                                    }
                                    None => {
                                        pos = m.end();
                                        break;
                                    }
                                }
                            }
                        }
                    };

                    if bad {
                        return Err(pos);
                    }
                }

                Ok(output)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::reglex;
    use regex::Regex;

    #[derive(Debug, PartialEq)]
    enum Token {
        Keyword,
        Number(u64),
        Left,
        Right,
    }

    #[test]
    fn it_doesnt_wor() {
        let lexer = reglex!(
            Token,
            "kw" => |_| Some(Token::Keyword),
            r"\d+" => |n| Some(Token::Number(n.parse().unwrap())),
            r"kw" => |_| Some(Token::Keyword),
            r"kw" => |_| Some(Token::Keyword),
            r"\s" => |_| None
        );

        assert_eq!(
            lexer("kw  { 12 53}".to_string()),
            Ok(vec![
                Token::Keyword,
                Token::Left,
                Token::Number(12),
                Token::Number(53),
                Token::Right
            ])
        );

        assert_eq!(lexer("kw m { 12 53}".to_string()), Err(3));
    }
}
