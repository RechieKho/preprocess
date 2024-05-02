use std::{borrow::Cow, char, collections::VecDeque};

use crate::{exception::Exception, token::Token};

const COMMENCEMENT_CHARACTER: char = '$';
const NEWLINE_CHARACTER: char = '\n';
const STRING_QUOTE_CHARACTER: char = '\'';
const OPEN_BRACKET_CHARACTER: char = '(';
const CLOSE_BRACKET_CHARACTER: char = ')';

pub trait Executor<'text> {
    fn get_value(&mut self, character: char) -> String;
    fn call(&mut self, arguments: &VecDeque<Token<'text>>) -> String;
}

pub struct DefaultExecutor {}

impl<'text> Executor<'text> for DefaultExecutor {
    fn call(&mut self, arguments: &VecDeque<Token<'text>>) -> String {
        let command = arguments.front().unwrap();
        if command.data == "say_hello" {
            return "Hello world".to_string();
        }

        String::default()
    }

    fn get_value(&mut self, character: char) -> String {
        if character == COMMENCEMENT_CHARACTER {
            return COMMENCEMENT_CHARACTER.to_string();
        }

        String::default()
    }
}

impl<'text> dyn Executor<'text> {
    pub fn execute(&mut self, text: &'text str) -> Result<String, Exception<'text>> {
        let mut result = String::with_capacity(text.len());
        let mut callee: VecDeque<Token<'text>> = VecDeque::with_capacity(16);

        let mut token_buffer = String::with_capacity(16);

        for (row, line) in text.split(NEWLINE_CHARACTER).enumerate() {
            let mut is_commencing = false;
            let mut is_quoting = false;

            for (column, character) in line.char_indices() {
                if is_commencing {
                    'commencing: {
                        if character == OPEN_BRACKET_CHARACTER {
                            callee.push_back(Token::<'text> {
                                data: character.to_string(),
                                line,
                                row,
                                column,
                            });
                            break 'commencing;
                        }

                        let output = self.get_value(character);
                        result.push_str(&output);
                    }
                    is_commencing = false;
                    continue;
                }

                if character == COMMENCEMENT_CHARACTER {
                    is_commencing = true;
                    continue;
                }

                if !callee.is_empty() {
                    'collecting_callee: {
                        if character == CLOSE_BRACKET_CHARACTER {
                            let mut arguments: VecDeque<Token<'text>> = VecDeque::with_capacity(16);

                            if !token_buffer.is_empty() {
                                arguments.push_front(Token::<'text> {
                                    data: token_buffer.clone(),
                                    line,
                                    row,
                                    column,
                                });
                                token_buffer.clear();
                            }

                            // TODO: There might be a better way to do this (https://users.rust-lang.org/t/best-way-to-drop-range-of-elements-from-front-of-vecdeque/31795).
                            loop {
                                let argument = callee.pop_back();
                                if argument.is_none() {
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
                                let argument = argument.unwrap();
                                // TODO: Please don't do excess allocation.
                                if &argument.data == &OPEN_BRACKET_CHARACTER.to_string() {
                                    break;
                                }
                                arguments.push_front(argument);
                            }

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

                            let output = self.call(&arguments);
                            result.push_str(&output);
                            break 'collecting_callee;
                        }

                        if is_quoting {
                            token_buffer.push(character);
                        } else if character == STRING_QUOTE_CHARACTER {
                            is_quoting = !is_quoting;
                        } else if character.is_whitespace() {
                            if !token_buffer.is_empty() {
                                callee.push_back(Token::<'text> {
                                    data: token_buffer.clone(),
                                    line,
                                    row,
                                    column,
                                });
                                token_buffer.clear();
                            }
                        } else {
                            token_buffer.push(character);
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
            } else if is_quoting {
                return Err(Exception {
                    message: Cow::Borrowed("Unterminated string."),
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

        Ok(result)
    }
}
