use magnus::{function, method, prelude::*, Error, Ruby};

fn hello() {
    println!("hello rust");
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    ruby.define_global_function("hello", function!(hello, 0));
    Ok(())
}
