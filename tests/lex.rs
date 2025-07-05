use rolang::lexer::*;

fn lex(src: &str) -> Vec<TokenType> {
    let res = TokenIterator::new(src).map(|tok| tok.ty).skip(1).collect();
    dbg!(&res);
    res
}

#[test]
fn test_numbers_simple() {
    let tokens = lex("42 0xFF 0xff 0b1010 3.14 3. 1e10 1.2e-3");
    for (n, token) in tokens.iter().enumerate() {
        match token {
            TokenType::Number { errors, .. } => {
                let errors = errors.bits();
                const NO_START_DIGITS_ERR: u8 = NumberErrorFlags::NO_START_DIGITS.bits();
                const FREE_DOT_ERR: u8 = NumberErrorFlags::FREE_DOT.bits();
                match (n, errors) {
                    (2, NO_START_DIGITS_ERR) => (),
                    (5, FREE_DOT_ERR) => (),
                    (_, 0) => {}
                    _ => panic!("Unexpected number error"),
                }
            }
            _ => panic!("Expected number token"),
        }
    }
    assert_eq!(tokens.len(), 8);
}

#[test]
fn test_strings_simple() {
    let tokens = lex("\"abc\" \"\" \"\\\"escaped\\\"\"");
    assert_eq!(tokens.len(), 3);
    for token in tokens {
        match token {
            TokenType::String { .. } => {}
            _ => panic!("Expected string"),
        }
    }
}

#[test]
fn test_full_expression() {
    let src = "mut x := fact 3 |> println \"Result: \" + x";
    let tokens = lex(src);

    let expected: Vec<&str> = vec![
        "Keyword", "Symbol", "Operator", "Symbol", "Number",
        "Operator", "Symbol", "String", "Operator", "Symbol"
    ];

    for (token, expected_str) in tokens.iter().zip(expected.iter()) {
        match (token, *expected_str) {
            (TokenType::Keyword { .. }, "Keyword") => {}
            (TokenType::Symbol { .. }, "Symbol") => {}
            (TokenType::Operator, "Operator") => {}
            (TokenType::Number { .. }, "Number") => {}
            (TokenType::String { .. }, "String") => {}
            _ => panic!("Unexpected token: {token:?}, expected {expected_str}"),
        }
    }
}
