use itertools::Itertools;

use crate::parser::Lexer;

pub struct CharacterEditorLexer {}

#[derive(Debug)]
pub enum CharacterEditorLexerError {
    UnexpectedCharacter {
        input: Option<String>,
        unexpected_char: char,
    },
}

pub enum CharacterEditorToken {
    Word(String),
    Number(i8),
}

impl CharacterEditorLexer {
    // Helper method to parse digits and add a number token
    fn parse_number<I>(
        input: &str,
        iter: &mut std::iter::Peekable<I>,
        tokens: &mut Vec<CharacterEditorToken>,
        is_negative: bool,
    ) -> Result<(), CharacterEditorLexerError>
    where
        I: Iterator<Item = char>,
    {
        let digits: String = iter
            .by_ref()
            .peeking_take_while(|c| c.is_ascii_digit())
            .collect();

        if Self::check_next(iter, |c| c.is_ascii_digit()) && !digits.is_empty() {
            if let Ok(mut number) = digits.parse::<i8>() {
                if is_negative {
                    number = -number;
                }
                tokens.push(CharacterEditorToken::Number(number));
            }
        } else if !digits.is_empty() {
            if let Some(&c) = iter.peek() {
                if is_negative {
                    return Err(CharacterEditorLexerError::UnexpectedCharacter {
                        input: Some(input.to_string()),
                        unexpected_char: c,
                    });
                } else {
                    return Err(CharacterEditorLexerError::UnexpectedCharacter {
                        input: Some(input.to_string()),
                        unexpected_char: c,
                    });
                }
            }
        }

        Ok(())
    }

    fn check_next<F, I>(iter: &mut std::iter::Peekable<I>, f: F) -> bool
    where
        F: Fn(char) -> bool,
        I: Iterator<Item = char>,
    {
        if let Some(&c) = iter.peek() {
            return f(c);
        }

        true // We default to true because this is the end of the input
    }
}

impl Lexer<CharacterEditorToken, CharacterEditorLexerError> for CharacterEditorLexer {
    fn lex(&self, input: String) -> Result<Vec<CharacterEditorToken>, CharacterEditorLexerError> {
        let mut tokens: Vec<CharacterEditorToken> = Vec::new();
        let mut iter = input.chars().peekable();

        while let Some(&ch) = iter.peek() {
            match ch {
                // pattern matching logic
                ch if ch.is_whitespace() => {
                    iter.next();
                }
                '-' => {
                    iter.next(); // Consume the '-'
                    Self::parse_number(&input, &mut iter, &mut tokens, true)?
                }
                '0'..='9' => {
                    // We've seen a digit, but haven't consumed it yet
                    Self::parse_number(&input, &mut iter, &mut tokens, false)?;
                }
                ch if ch.is_alphabetic() => {
                    // First, consume the alphabetic character we just peeked at
                    let first_char = iter.next().unwrap();

                    // Then collect the rest of the alphanumeric characters
                    let mut chars = first_char.to_string();
                    let rest: String = iter
                        .by_ref()
                        .peeking_take_while(|&c| c.is_alphanumeric() || c == '_')
                        .collect();
                    chars.push_str(&rest);

                    if Self::check_next(&mut iter, |c| c.is_whitespace()) {
                        tokens.push(CharacterEditorToken::Word(chars));
                    } else if let Some(&c) = iter.peek() {
                        return Err(CharacterEditorLexerError::UnexpectedCharacter {
                            input: Some(input),
                            unexpected_char: c,
                        });
                    }
                }
                _ => {
                    return Err(CharacterEditorLexerError::UnexpectedCharacter {
                        input: Some(input),
                        unexpected_char: ch,
                    })
                }
            }
        }

        Ok(tokens)
    }
}
