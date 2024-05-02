use context::Context;
use executor::Executor;

mod context;
mod exception;
mod executor;
mod token;

fn main() {
    let mut executor: Box<dyn Executor> = Box::new(Context::default());
    let result = executor.execute("$(set hello world)$(hello)");

    match result {
        Err(exception) => {
            println!("{}", exception.message);
        }
        Ok(result) => {
            println!("{}", result);
        }
    };
}
