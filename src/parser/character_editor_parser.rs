use crate::app::Event;
use crate::parser::character_editor_lexer::CharacterEditorToken;
use crate::parser::Parser;

pub struct CharacterEditorParser {}

pub enum CharacterEditorParserError {
    FirstTokenCannotBeNumber(i8),
    NoTokensProvided,
    UnknownCommand(String),
    ExtraneousArgumentsProvided(Vec<CharacterEditorToken>),
}

impl CharacterEditorParser {
    fn map_event(
        command_specifier: String,
        tokens: Vec<CharacterEditorToken>,
    ) -> Result<Event, CharacterEditorParserError> {
        match command_specifier.as_str() {
            "Exit" | "exit" => {
                if tokens.len() == 1 {
                    Ok(Event::Exit)
                } else {
                    Err(CharacterEditorParserError::ExtraneousArgumentsProvided(
                        tokens,
                    ))
                }
            }
            _ => Err(CharacterEditorParserError::UnknownCommand(
                command_specifier,
            )),
        }
    }
}

impl Parser<CharacterEditorToken, Event, CharacterEditorParserError> for CharacterEditorParser {
    fn parse(
        &self,
        tokens: Vec<CharacterEditorToken>,
    ) -> Result<Event, CharacterEditorParserError> {
        // Here we support a number of commands, and create events to send.
        if !tokens.is_empty() {
            if let Some(first_token) = tokens.first() {
                match first_token {
                    CharacterEditorToken::Word(command) => {
                        return CharacterEditorParser::map_event(command.to_string(), tokens);
                    }
                    CharacterEditorToken::Number(number) => {
                        return Err(CharacterEditorParserError::FirstTokenCannotBeNumber(
                            *number,
                        ));
                    }
                }
            }
        }

        Err(CharacterEditorParserError::NoTokensProvided)
    }
}
