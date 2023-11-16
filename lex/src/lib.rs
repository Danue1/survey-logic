mod context;

use context::Context;
use syntax_kind::SyntaxKind;

pub fn lex(source: &str) -> impl Iterator<Item = (SyntaxKind, String)> + '_ {
    let mut context = Context::new(source);

    std::iter::from_fn(move || {
        let start = context.index();
        let c = context.bump()?;
        if let Some(kind) = punctuation(c) {
            return Some((kind, c.to_string()));
        }

        if c.is_ascii_whitespace() {
            loop {
                match context.peek() {
                    Some(c) if c.is_ascii_whitespace() => context.advance(),
                    _ => break,
                }
            }
            return Some((SyntaxKind::WHITESPACE, context.slice(start)));
        }

        match c {
            '0'..='9' => {
                loop {
                    match context.peek() {
                        Some(c) if c.is_ascii_digit() => context.advance(),
                        _ => break,
                    }
                }
                Some((SyntaxKind::NUMERIC, context.slice(start)))
            }
            quote @ ('\'' | '"') => {
                while let Some(c) = context.peek() {
                    context.advance();
                    if c == quote {
                        break;
                    }
                }
                Some((SyntaxKind::STRING, context.slice(start)))
            }
            'a'..='z' | 'A'..='Z' => {
                loop {
                    match context.peek() {
                        Some(c) if c.is_alphabetic() => context.advance(),
                        _ => break,
                    }
                }
                Some((SyntaxKind::IDENTIFIER, context.slice(start)))
            }
            _ => Some((SyntaxKind::UNKNOWN, c.to_string())),
        }
    })
}

const fn punctuation(c: char) -> Option<SyntaxKind> {
    match c {
        '(' => Some(SyntaxKind::LEFT_PAREN),
        ')' => Some(SyntaxKind::RIGHT_PAREN),
        '[' => Some(SyntaxKind::LEFT_BRACKET),
        ']' => Some(SyntaxKind::RIGHT_BRACKET),
        '<' => Some(SyntaxKind::LEFT_CHEVRON),
        '>' => Some(SyntaxKind::RIGHT_CHEVRON),
        '=' => Some(SyntaxKind::EQUAL),
        '!' => Some(SyntaxKind::EXCLAMATION),
        '.' => Some(SyntaxKind::DOT),
        ',' => Some(SyntaxKind::COMMA),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_tokens {
        ($(
            $name:ident($source:expr) => $tokens:expr,
        )+) => {
            $(
                #[test]
                fn $name() {
                    let tokens: Vec<(SyntaxKind, String)> = lex($source).collect();
                    assert_eq!(tokens, $tokens);
                }
            )+
        };
    }

    assert_tokens! {
        punctuation("()[]<>=!., \n\r\t") => vec![
            (SyntaxKind::LEFT_PAREN, "(".to_string()),
            (SyntaxKind::RIGHT_PAREN, ")".to_string()),
            (SyntaxKind::LEFT_BRACKET, "[".to_string()),
            (SyntaxKind::RIGHT_BRACKET, "]".to_string()),
            (SyntaxKind::LEFT_CHEVRON, "<".to_string()),
            (SyntaxKind::RIGHT_CHEVRON, ">".to_string()),
            (SyntaxKind::EQUAL, "=".to_string()),
            (SyntaxKind::EXCLAMATION, "!".to_string()),
            (SyntaxKind::DOT, ".".to_string()),
            (SyntaxKind::COMMA, ",".to_string()),
            (SyntaxKind::WHITESPACE, " \n\r\t".to_string()),
        ],
        numeric("1234567890.1234567890") => vec![
            (SyntaxKind::NUMERIC, "1234567890".to_string()),
            (SyntaxKind::DOT, ".".to_string()),
            (SyntaxKind::NUMERIC, "1234567890".to_string()),
        ],
        string("'abc'\"def\"") => vec![
            (SyntaxKind::STRING, "'abc'".to_string()),
            (SyntaxKind::STRING, "\"def\"".to_string()),
        ],
        identifier("abc def") => vec![
            (SyntaxKind::IDENTIFIER, "abc".to_string()),
            (SyntaxKind::WHITESPACE, " ".to_string()),
            (SyntaxKind::IDENTIFIER, "def".to_string()),
        ],
        question_parameter("q1 q2") => vec![
            (SyntaxKind::IDENTIFIER, "q".to_string()),
            (SyntaxKind::NUMERIC, "1".to_string()),
            (SyntaxKind::WHITESPACE, " ".to_string()),
            (SyntaxKind::IDENTIFIER, "q".to_string()),
            (SyntaxKind::NUMERIC, "2".to_string()),
        ],
    }
}
