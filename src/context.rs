use std::collections::{BTreeMap, VecDeque};

use crate::{
    exception::Exception,
    executor::{Executor, COMMENCEMENT_CHARACTER},
    token::Token,
};

use self::set::Set;

mod set;

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
    store: Store,
    registry: CalleeRegistry<'text>,
}

impl<'text> Context<'text> {
    pub fn register_callee(&mut self, callee: Box<dyn Callee<'text>>) {
        self.registry.insert(callee.name(), callee);
    }
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

impl<'text> Executor<'text> for Context<'text> {
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

    fn get(&mut self, character: char) -> Result<String, Exception<'text>> {
        if character == COMMENCEMENT_CHARACTER {
            return Ok(COMMENCEMENT_CHARACTER.to_string());
        }

        Ok(String::default())
    }
}
