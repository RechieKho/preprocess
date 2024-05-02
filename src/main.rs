use crate::executor::{DefaultExecutor, Executor};

mod exception;
mod executor;
mod token;

fn main() {
    let mut executor: Box<dyn Executor> = Box::new(DefaultExecutor {});
    let result = executor.execute("$(say_hello) $$");

    match result {
        Err(exception) => {
            println!("{}", exception.message);
        }
        Ok(result) => {
            println!("{}", result);
        }
    };
}
