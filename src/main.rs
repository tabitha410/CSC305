///associate greetings module with this crate
mod greetings;
mod how_you_hold_data_for_operations;
use how_you_hold_data_for_operations::primitives::scalar;
use how_you_hold_data_for_operations::derived::user_defined;
use how_you_hold_data_for_operations::primitives::compound;

// use greetings::default_greeting;
use greetings::{default_greeting,spanish,french,english};
fn main() {

   let greetings: &str = "Hello there!";
   println!("{}",greetings);

    println!("Hello, world!");
    println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", english::default_greeting());
    println!("{}", hello_world_lib::greeting_from_lib());
    scalar::run();
    user_defined::run();
    compound::run();
}