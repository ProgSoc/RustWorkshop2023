use std::{thread::sleep, time::Duration};

fn get_thing() -> i32 {
  return 42;
}

/// Walk-through of a Wikipedia example, no-boilerplate example and more
fn main() {
    // Activity
    // Compile a Rust program on your machine.
    println!("🤖 Beep...");
    println!("...Boop! 🤖");
    println!("Hello, World! 👋🌏\n");

    // Activity
    // Variables
    let x;
    x = 4;
    // Inline, not out of place in Javascript
    let x = 42;
    // Types, like Typescript
    let x: i32;
    x = 42;
    let x: i32 = 42;
    // i32 is a signed 32-bit integer
    // if you've used C on a modern computer, that's int!
    //
    let x;
    // this will work on C! but what does it do??
    // println!(x);
    x = 42;
    // println!(x);
    //
    let _ = 42;
    let _ = get_thing();

    // Activity
    // Control-Flow
    // let values = vec![1, 2, 3, 4];
    let mut values = vec![1, 2, 3, 4, 11];
    //
    // for value in values {
    for value in &values {
        println!("value = {}", value);
    }
    //
    if values.len() > 5 {
        println!("🦣 List is longer than five items");
    }
    //
    // Pattern matching
    match values.len() {
        0 => println!("🕳️ Empty"),
        1 => println!("🐪 One value"),
        // pattern matching can use ranges of integers
        2..=10 => println!("🐫 Between two and ten values"),
        11 => println!("🦀 Eleven values"),
        // A `_` pattern is called a "wildcard", it matches any value
        _ => println!("✨ Many values"),
    }
    //
    while let Some(value) = values.pop() {
        println!("👋 Goodbye value = {value} !");
    }

    // Brief expressions
    let x = {
        let y = 1;
        let z = 2;
        y + z;
    };

    // Activity
    // Using more than what's already here (use std).
    println!("Yawwnnn 🥱 zzzzz 😴");
    sleep(Duration::from_secs(5));
    println!("Oooh 🤭, hopefully I didn't miss anything 😊");
}
