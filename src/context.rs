use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
};

use crate::{
    exception::Exception,
    executor::{Executor, COMMENCEMENT_CHARACTER},
    token::Token,
};

#[derive(Default)]
pub struct Context {
    stack: VecDeque<HashMap<String, String>>,
}

impl Context {
    fn get_current_stack(&mut self) -> &mut HashMap<String, String> {
        if self.stack.len() == 0 {
            self.stack.push_back(HashMap::default());
        }

        self.stack.back_mut().unwrap()
    }

    pub fn decl(&mut self, key: String, value: String) {
        let current_stack = self.get_current_stack();
        current_stack.insert(key, value);
    }

    pub fn set(&mut self, key: String, value: String) {
        let position = self.stack.iter().rposition(|map| map.contains_key(&key));
        if position.is_none() {
            self.decl(key, value);
            return;
        }
        let position = position.unwrap();
        self.stack[position].insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&String> {
        let position = self.stack.iter().rposition(|map| map.contains_key(key));
        if position.is_none() {
            return None;
        }
        let position = position.unwrap();
        return self.stack[position].get(key);
    }
}

impl<'text> Executor<'text> for Context {
    fn call(
        &mut self,
        head: Token<'text>,
        mut body: VecDeque<Token<'text>>,
    ) -> Result<String, Exception<'text>> {
        if head.data == "decl" {
            if body.len() != 2 {
                return Err(Exception {
                    message: Cow::Borrowed("`decl` expect 2 arguments."),
                    token: head,
                });
            }
            let key = body.pop_front().unwrap();
            let value = body.pop_front().unwrap();
            self.decl(key.data, value.data);
            return Ok(String::default());
        }

        if head.data == "set" {
            if body.len() != 2 {
                return Err(Exception {
                    message: Cow::Borrowed("`set` expect 2 arguments."),
                    token: head,
                });
            }
            let key = body.pop_front().unwrap();
            let value = body.pop_front().unwrap();
            self.set(key.data, value.data);
            return Ok(String::default());
        }

        let value = self.get(&head.data);
        if value.is_none() {
            Ok(String::default())
        } else {
            Ok(value.unwrap().clone())
        }
    }

    fn get_value(&mut self, character: char) -> Result<String, Exception<'text>> {
        if character == COMMENCEMENT_CHARACTER {
            return Ok(COMMENCEMENT_CHARACTER.to_string());
        }

        Ok(String::default())
    }
}
