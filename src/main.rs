use context::Context;
use executor::Executor;

mod context;
mod exception;
mod executor;
mod token;

fn main() {
    let mut executor: Box<dyn Executor> = Box::new(Context {});
    let result = executor.execute("$(say_hello $(say_hello 1 2 3)) $$");

    match result {
        Err(exception) => {
            println!("{}", exception.message);
        }
        Ok(result) => {
            println!("{}", result);
        }
    };
}
