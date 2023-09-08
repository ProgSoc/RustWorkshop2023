use input_macro::input;
use rand::{thread_rng, Rng};
use toml::Value;

fn main() {
    // Activity
    // Use an external crate to add some interactivity.
    println!("Hello, {}!", input!("What's your name? "));

    // Activity
    // Lightly using generics and results.
    println!("Let's do some maths...");
    let x: isize = input!("Give me a number to add to 2: ").parse().unwrap();
    println!("2 + {x} = {}", 2 + x);

    // Activity Game
    // Guess the computer's number
    let mut rng = thread_rng();
    let x = rng.gen_range(1..=10);
    let guess = input!("Guess my numbered between 1-10: ").parse().unwrap();
    println!("{} {}", if x == guess { "You got it! It was" } else {"Nope! It was"}, guess);

    // Activity
    // Load a TOML file
    if let Ok(val) = toml::from_str::<Value>(include_str!("../Cargo.toml")) {
        println!("Program Description: {}", val["package"]["description"]);
    }
}
