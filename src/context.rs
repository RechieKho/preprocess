use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
};

use crate::{exception::Exception, token::Token};

use self::set::Set;

mod set;

const INITIAL_CAPACITY: usize = 16;

pub const COMMENCEMENT_CHARACTER: char = '@';
pub const NEWLINE_CHARACTER: char = '\n';
pub const OPEN_BRACKET_CHARACTER: char = '(';
pub const CLOSE_BRACKET_CHARACTER: char = ')';
pub const SPACE_CHARACTER: char = ' ';
pub const UNDERSCORE_CHARACTER: char = '_';

type CalleeRegistry<'text> = BTreeMap<&'static str, Box<dyn Callee<'text>>>;
type Store = BTreeMap<String, String>;

pub trait Callee<'text> {
    fn name(&self) -> &'static str;
    fn call(
        &mut self,
        store: &mut Store,
        body: VecDeque<Token<'text>>,
    ) -> Result<String, Exception<'text>>;
}

pub struct Context<'text> {
    pub store: Store,
    registry: CalleeRegistry<'text>,
}

impl<'text> Default for Context<'text> {
    fn default() -> Self {
        let mut context = Context::<'text> {
            store: Store::default(),
            registry: CalleeRegistry::<'text>::default(),
        };

        context.register_callee(Box::new(Set::default()));

        context
    }
}

impl<'text> Context<'text> {
    fn call(
        &mut self,
        head: Token<'text>,
        body: VecDeque<Token<'text>>,
    ) -> Result<String, Exception<'text>> {
        let callee = self.registry.get_mut(head.data.as_str());
        match callee {
            None => {
                let value = self.store.get(head.data.as_str());
                match value {
                    None => Ok(String::default()),
                    Some(value) => Ok(value.clone()),
                }
            }
            Some(callee) => callee.call(&mut self.store, body),
        }
    }

    fn special(&mut self, character: char) -> Result<String, Exception<'text>> {
        if character == COMMENCEMENT_CHARACTER {
            return Ok(COMMENCEMENT_CHARACTER.to_string());
        } else if character == UNDERSCORE_CHARACTER {
            return Ok(SPACE_CHARACTER.to_string());
        }

        Ok(String::default())
    }

    pub fn register_callee(&mut self, callee: Box<dyn Callee<'text>>) {
        self.registry.insert(callee.name(), callee);
    }

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

                        let output = self.special(character)?;
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

        result.pop();

        Ok(result)
    }
}

#[test]
fn test_replacement_call() {
    let mut context = Context::default();
    context
        .store
        .insert("hello".to_string(), "world".to_string());
    let result = context.execute("@(hello)");
    assert_eq!(result.unwrap(), "world".to_string());
}
