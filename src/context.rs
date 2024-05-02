use std::collections::VecDeque;

use crate::{
    executor::{Executor, COMMENCEMENT_CHARACTER},
    token::Token,
};

pub struct Context {}

impl<'text> Executor<'text> for Context {
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
