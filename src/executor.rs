use std::{borrow::Cow, char, collections::VecDeque};

use crate::{exception::Exception, token::Token};

const INITIAL_CAPACITY: usize = 16;

pub const COMMENCEMENT_CHARACTER: char = '@';
pub const NEWLINE_CHARACTER: char = '\n';
pub const OPEN_BRACKET_CHARACTER: char = '(';
pub const CLOSE_BRACKET_CHARACTER: char = ')';

pub trait Executor<'text> {
    fn get(&mut self, character: char) -> Result<String, Exception<'text>>;
    fn call(
        &mut self,
        head: Token<'text>,
        body: VecDeque<Token<'text>>,
    ) -> Result<String, Exception<'text>>;
}

impl<'text> dyn Executor<'text> {
    pub fn execute(&mut self, text: &'text str) -> Result<String, Exception<'text>> {
        let mut result = String::with_capacity(text.len());
        let mut tokens: VecDeque<Token<'text>> = VecDeque::with_capacity(INITIAL_CAPACITY);

        let mut token_data_buffer = String::with_capacity(INITIAL_CAPACITY);

        for (row, line) in text.split(NEWLINE_CHARACTER).enumerate() {
            let mut is_commencing = false;

            for (column, character) in line.char_indices() {
                if is_commencing {
                    'commencing: {
                        if character == OPEN_BRACKET_CHARACTER {
                            tokens.push_back(Token::<'text> {
                                data: character.to_string(),
                                line,
                                row,
                                column,
                            });
                            break 'commencing;
                        }

                        let output = self.get(character)?;
                        if tokens.is_empty() {
                            result.push_str(&output);
                        } else {
                            tokens.push_back(Token::<'text> {
                                data: output,
                                line,
                                row,
                                column,
                            })
                        }
                    }
                    is_commencing = false;
                    continue;
                }

                if character == COMMENCEMENT_CHARACTER {
                    is_commencing = true;
                    continue;
                }

                if !tokens.is_empty() {
                    'collecting_tokens: {
                        if character == OPEN_BRACKET_CHARACTER {
                            tokens.push_back(Token::<'text> {
                                data: character.to_string(),
                                line,
                                row,
                                column,
                            });
                            break 'collecting_tokens;
                        }

                        if character == CLOSE_BRACKET_CHARACTER {
                            let mut arguments: VecDeque<Token<'text>> =
                                VecDeque::with_capacity(INITIAL_CAPACITY);

                            if !token_data_buffer.is_empty() {
                                tokens.push_back(Token::<'text> {
                                    data: token_data_buffer.clone(),
                                    line,
                                    row,
                                    column,
                                });
                                token_data_buffer.clear();
                            }

                            let position = tokens
                                .iter()
                                .rposition(|token| token == &OPEN_BRACKET_CHARACTER);
                            if position.is_none() {
                                return Err(Exception {
                                    message: Cow::Borrowed("Excess closing bracket."),
                                    token: Token::<'text> {
                                        data: character.to_string(),
                                        line,
                                        row,
                                        column,
                                    },
                                });
                            }
                            let position = position.unwrap();
                            arguments.extend(tokens.drain(position..).skip(1));

                            if arguments.is_empty() {
                                return Err(Exception {
                                    message: Cow::Borrowed("Empty call."),
                                    token: Token::<'text> {
                                        data: character.to_string(),
                                        line,
                                        row,
                                        column,
                                    },
                                });
                            }

                            let head = arguments.pop_front().unwrap();

                            let output = self.call(head, arguments)?;
                            if tokens.is_empty() {
                                result.push_str(&output);
                            } else {
                                tokens.push_back(Token::<'text> {
                                    data: output,
                                    line,
                                    row,
                                    column,
                                });
                            }
                            break 'collecting_tokens;
                        }

                        if character.is_whitespace() {
                            if !token_data_buffer.is_empty() {
                                tokens.push_back(Token::<'text> {
                                    data: token_data_buffer.clone(),
                                    line,
                                    row,
                                    column,
                                });
                                token_data_buffer.clear();
                            }
                        } else {
                            token_data_buffer.push(character);
                        }
                    }

                    continue;
                }

                result.push(character);
            }

            if is_commencing {
                return Err(Exception {
                    message: Cow::Borrowed("Unterminated commencement character."),
                    token: Token::<'text> {
                        data: NEWLINE_CHARACTER.to_string(),
                        line,
                        row,
                        column: line.len() - 1,
                    },
                });
            }

            result.push(NEWLINE_CHARACTER);
        }

        if !tokens.is_empty() {
            return Err(Exception {
                message: Cow::Borrowed("Unterminated bracket."),
                token: tokens.pop_back().unwrap(),
            });
        }

        Ok(result)
    }
}
