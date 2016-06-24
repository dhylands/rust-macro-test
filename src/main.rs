#[macro_use]
mod utils;    // Note: This MUST come before mod foo;
mod foo;

fn main() {
    println!("Hello, world! 5 * 5 = {}", five_times!(5));
    foo::foo();
}
