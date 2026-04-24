#[derive(Debug, PartialEq)]
enum Token {
    Lit(String),
    Var(String),
}

#[derive(PartialEq)]
enum ParseState {
    Lit,
    Var,
}

fn parse_template(text: &str) -> Vec<Token> {
    let mut res = Vec::new();
    let mut buf = String::new();
    let mut state = ParseState::Lit;
    for c in text.to_string().chars() {
        match state {
            ParseState::Lit => {
                if c == '{' {
                    if !buf.is_empty() {
                        res.push(Token::Lit(buf.to_string()));
                        buf.clear();
                    }
                    state = ParseState::Var;
                    continue;
                }
            }
            ParseState::Var => {
                if c == '}' {
                    if !buf.is_empty() {
                        res.push(Token::Var(buf.to_string()));
                        buf.clear();
                    }
                    state = ParseState::Lit;
                    continue;
                }
            }
        }
        buf.push(c);
    }
    if !buf.is_empty() {
        if state == ParseState::Var {
            // Unclosed parentheses
            todo!();
        }
        res.push(Token::Lit(buf.to_string()));
    }
    res
}

pub fn construct_string(
    template: &str,
    access_hash: &std::collections::HashMap<String, usize>,
    data_row: &Vec<String>,
) -> String {
    let mut res = String::new();
    let toks = parse_template(template);
    for tok in toks {
        match tok {
            Token::Lit(str) => {
                res.push_str(&str);
            }
            Token::Var(str) => {
                let value = &data_row[*access_hash
                    .get(&str as &str)
                    .unwrap_or_else(|| panic!("Error: {} is not found in the source hash.", str))];
                res.push_str(value);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn simple_input() {
        let input = "The quick brown for jumps over the {a}";
        let tokens = parse_template(input);
        assert_eq!(
            tokens,
            vec![
                Token::Lit("The quick brown for jumps over the ".to_string()),
                Token::Var("a".to_string()),
            ]
        );
    }

    #[test]
    fn lit_with_two_var() {
        let input = "The {quick} brown for jumps over the {a}";
        let tokens = parse_template(input);
        assert_eq!(
            tokens,
            vec![
                Token::Lit("The ".to_string()),
                Token::Var("quick".to_string()),
                Token::Lit(" brown for jumps over the ".to_string()),
                Token::Var("a".to_string()),
            ]
        );
    }

    #[test]
    fn lit_with_three_var() {
        let input = "The {quick} brown {for} jumps over the {a}";
        let tokens = parse_template(input);
        assert_eq!(
            tokens,
            vec![
                Token::Lit("The ".to_string()),
                Token::Var("quick".to_string()),
                Token::Lit(" brown ".to_string()),
                Token::Var("for".to_string()),
                Token::Lit(" jumps over the ".to_string()),
                Token::Var("a".to_string()),
            ]
        );
    }

    #[test]
    fn lit_with_no_var() {
        let input = "The quick brown for jumps over the a";
        let tokens = parse_template(input);
        assert_eq!(
            tokens,
            vec![Token::Lit(
                "The quick brown for jumps over the a".to_string()
            ),]
        );
    }

    #[test]
    fn one_var_with_no_lit() {
        let input = "{student_name}";
        let tokens = parse_template(input);
        assert_eq!(tokens, vec![Token::Var("student_name".to_string()),]);
    }

    #[test]
    fn two_var_with_no_lit() {
        let input = "{student_name}{date}";
        let tokens = parse_template(input);
        assert_eq!(
            tokens,
            vec![
                Token::Var("student_name".to_string()),
                Token::Var("date".to_string()),
            ]
        );
    }

    #[test]
    fn two_var_with_space() {
        let input = "{student_name} {date}";
        let tokens = parse_template(input);
        assert_eq!(
            tokens,
            vec![
                Token::Var("student_name".to_string()),
                Token::Lit(" ".to_string()),
                Token::Var("date".to_string()),
            ]
        );
    }

    #[test]
    fn generate_test() {
        let access_hash = HashMap::from([("name".to_string(), 0), ("date".to_string(), 1), ("level".to_string(), 2)]);
        let data = vec![
            "oihv".to_string(),
            "2026/04/23".to_string(),
            "HSK 4".to_string(),
        ];
        let input =
            "{name} is currently writing CertGen at {date}. He have {level} certificate.";
        assert_eq!(
            construct_string(input, &access_hash, &data),
            "oihv is currently writing CertGen at 2026/04/23. He have HSK 4 certificate."
        )
    }
}
