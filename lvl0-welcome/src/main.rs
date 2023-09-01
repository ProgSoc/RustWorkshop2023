use input_macro::input;
use std::{thread::sleep, time::Duration};

fn main() {
    // Activity 1 (Playground)
    // Compile a Rust program on your machine.
    println!("🤖 Beep...");
    println!("...Boop! 🤖");
    println!("Hello, World! 👋🌏\n");

    // Activity 2 (Playground)
    // Using more than what's already here (use std).
    println!("Yawwnnn 🥱 zzzzz 😴");
    sleep(Duration::from_secs(3));
    println!("Oooh 🤭, hopefully I didn't miss anything 😊");

    // Activity 3
    // Use an external crate to add some interactivity.
    println!("Hello, {}!", input!("What's your name? "));

    // Activity 4
    // Lightly using generics and results.
    println!("Let's do some maths...");
    let x: isize = input!("Give me a number to add to 2: ").parse().unwrap();
    println!("2 + {x} = {}", 2 + x);
}
