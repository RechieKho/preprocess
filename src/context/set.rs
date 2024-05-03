use std::collections::VecDeque;

use crate::{exception::Exception, token::Token};

use super::{Callee, Store};

#[derive(Default)]
pub(super) struct Set {}

impl<'text> Callee<'text> for Set {
    fn name(&self) -> &'static str {
        "set"
    }

    fn call(
        &mut self,
        store: &mut Store,
        mut body: VecDeque<Token<'text>>,
    ) -> Result<String, Exception<'text>> {
        let key = body.pop_front().unwrap();
        let mut value = body
            .drain(..)
            .fold(String::default(), |mut value, argument| {
                value.push_str(argument.data.as_str());
                value.push(' ');
                value
            });
        value.pop();
        store.insert(key.data, value);
        Ok(String::default())
    }
}
