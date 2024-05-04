use std::borrow::Cow;

use crate::token::Token;

#[derive(Debug)]
pub struct Exception<'text> {
    pub message: Cow<'static, str>,
    pub token: Token<'text>,
}
