use context::Context;
use executor::Executor;

mod context;
mod exception;
mod executor;
mod token;

fn main() {
    let mut context = Context::default();
    let result =
        (&mut context as &mut dyn Executor).execute("$(set hello 'world asdfasd')$(hello) Man");

    match result {
        Err(exception) => {
            println!("{}", exception.message);
        }
        Ok(result) => {
            println!("{}", result);
        }
    };
}
