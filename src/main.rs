use std::{
    env,
    io::{self, Read, Write},
};

use context::Context;

mod context;
mod exception;
mod token;

fn main() -> io::Result<()> {
    let mut input = String::with_capacity(128);
    io::stdin().read_to_string(&mut input)?;
    {
        let mut context = Context::default();
        context.store.extend(env::vars());

        let result = context.execute(input.as_str());

        match result {
            Err(exception) => io::stderr().write_fmt(format_args!("{}", exception.message)),
            Ok(result) => io::stdout().write_fmt(format_args!("{}", result)),
        }
    }
}
