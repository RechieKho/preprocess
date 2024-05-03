use context::Context;
use executor::Executor;

mod context;
mod exception;
mod executor;
mod token;

fn main() {
    let mut context = Context::default();
    let result = (&mut context as &mut dyn Executor)
        .execute("@(set hello world @_@_@_@_@_@_ asdfasd)@(set goodbye   @(hello))@(goodbye)Man");

    match result {
        Err(exception) => {
            println!("{}", exception.message);
        }
        Ok(result) => {
            println!("{}", result);
        }
    };
}
