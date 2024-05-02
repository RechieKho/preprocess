use std::borrow::Cow;

use crate::token::Token;

pub struct Exception<'text> {
    pub message: Cow<'static, str>,
    pub token: Token<'text>,
}
