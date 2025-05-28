use itertools::Itertools;

use crate::parser::Lexer;

pub struct CharacterEditorLexer {}

#[derive(Debug)]
pub enum CharacterEditorLexerError {
    UnexpectedCharacter(char),
}

pub enum CharacterEditorToken {
    Word(String),
    Number(i8),
}

impl CharacterEditorLexer {
    // Helper method to parse digits and add a number token
    fn parse_number<I>(
        iter: &mut std::iter::Peekable<I>,
        tokens: &mut Vec<CharacterEditorToken>,
        is_negative: bool,
    ) where
        I: Iterator<Item = char>,
    {
        let digits: String = iter
            .by_ref()
            .peeking_take_while(|c| c.is_ascii_digit())
            .collect();

        if !digits.is_empty() {
            if let Ok(mut number) = digits.parse::<i8>() {
                if is_negative {
                    number = -number;
                }
                tokens.push(CharacterEditorToken::Number(number));
            }
        }
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
                    Self::parse_number(&mut iter, &mut tokens, true);
                }
                '0'..='9' => {
                    // We've seen a digit, but haven't consumed it yet
                    Self::parse_number(&mut iter, &mut tokens, false);
                }
                ch if ch.is_alphabetic() => {
                    println!("{:?}", iter.peek());
                    // First, consume the alphabetic character we just peeked at
                    let first_char = iter.next().unwrap();

                    // Then collect the rest of the alphanumeric characters
                    let mut chars = first_char.to_string();
                    let rest: String = iter
                        .by_ref()
                        .take_while(|&c| c.is_alphanumeric() || c == '_')
                        .collect();

                    chars.push_str(&rest);
                    tokens.push(CharacterEditorToken::Word(chars));
                }
                _ => {
                    if let Some(c) = iter.next() {
                        return Err(CharacterEditorLexerError::UnexpectedCharacter(c));
                    }
                }
            }
        }

        Ok(tokens)
    }
}
