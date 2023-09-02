#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_assignments)]

// Main function, runs the program
fn main() {
    // Print to console
    println!("Hello, world!");

    // Formatted prints
    println!("Hello, {}!", "Jim");
}

#[cfg(disabled)]
fn make_variables() {
    // Variables are initialized with "let"
    let variable = 1;

    // You can explicitly set the type of a variable, though it's not necessary
    let var_with_type: i32 = 1;

    // Variables are immutable by default, but can be made mutable with "mut"
    let mut mutable_var = 1;
    mutable_var = 2;

    // Variables can be "shadowed", which means you can redeclare them with the same name
    let shadowed_var = 1;
    let shadowed_var = "foo";
    println!("{}", shadowed_var); // Prints "foo"
}

#[cfg(disabled)]
fn control_flow() {
    // If statements
    let x = 1;
    if x == 1 {
        println!("x is 1");
    } else if x == 2 {
        println!("x is 2");
    } else {
        println!("x is not 1 or 2");
    }

    // Loops
    let mut i = 0;
    loop {
        i += 1;
        if i == 10 {
            break;
        }
    }

    // While loops
    let mut i = 0;
    while i < 10 {
        i += 1;
    }

    // For loops
    for i in 0..10 {
        println!("{}", i);
    }

    // For loops with iterators
    let arr = [1, 2, 3];
    for i in arr.iter() {
        println!("{}", i);
    }

    // Match statements
    let x = 1;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        4..=10 => println!("4 <= x <= 10"),

        _ => println!("x is something else"),
    }
}

// Functions have arguments and return types
#[cfg(disabled)]
fn functions(arg1: i32, arg2: i32) -> i32 {
    return arg1 + arg2;
}

// Function with an implicit return value
#[cfg(disabled)]
fn implicit_returns(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(disabled)]
fn check_even(n: i32) -> bool {
    // The last line in any block without a semicolon is implicitly returned
    if n % 2 == 0 {
        // This block returns true
        true
    } else {
        // This block returns false
        false
    }
    // The if statement is an expression and doesn't have a semicolon so it's returned
}

#[cfg(disabled)]
// Loops can be expressions as well, with the break statement creating the return value
fn find_and_multiply(numbers: Vec<i32>, target: i32) -> Option<i32> {
    // This loop looks for the target number in the list
    // and if found, multiplies it by 2 and returns.
    // If not found, it returns None.
    let result = loop {
        if numbers.is_empty() {
            break None;
        }

        let number = numbers.pop().unwrap();
        if number == target {
            break Some(number * 2);
        }
    };

    result
}

// Struct items are accessed with .
#[cfg(disabled)]
struct Foo {
    x: i32,
}

#[cfg(disabled)]
fn structs() {
    let mut foo = Foo { x: 1 };
    println!("{}", foo.x); // Prints 1

    foo.x = 2;
    println!("{}", foo.x); // Prints 2
}

// Namespace items are accessed with ::
#[cfg(disabled)]
mod foo {
    pub fn bar() {}
}

#[cfg(disabled)]
fn namespaces() {
    foo::bar();

    // You can also use "use" to bring items into scope
    use foo::bar;
    bar();
}

// The standard library contains all the useful stuff
#[cfg(disabled)]
fn standard_library() {
    // Vectors are like arrays, but can grow and shrink
    let mut vec = Vec::new();
    vec.push(1); // vec is now [1]
    vec.push(2); // vec is now [1, 2]
    vec.push(3); // vec is now [1, 2, 3]
    vec.push(4); // vec is now [1, 2, 3, 4]
    vec.remove(0); // vec is now [2, 3, 4]

    // Vecs can also be initialized with a macro
    let vec = vec![1, 2, 3, 4];

    // Strings are UTF-8 encoded
    let mut string = String::new();
    string.push_str("foo");
    string.push('b'); // string is now "foob"
    string.remove(0); // string is now "oob"

    // Hashmaps are like dictionaries
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("foo", "bar"); // map is now {"foo": "bar"}
    map.remove("foo"); // map is now {}

    // And many more things

    // The standard library also contains useful macros
    println!("{} {}", file!(), line!()); // Prints "src/main.rs 1"
}

// The generic types of structs and functions are often inferred
#[cfg(disabled)]
fn generics() {
    struct Foo<T> {
        x: T,
    }

    let foo = Foo { x: 1 };
    let foo = Foo { x: "foo" };

    let mut vec = Vec::new();
    vec.push(1);

    let mut vec2 = Vec::new();
    vec2.push("foo");
    vec2.push(1); // Error
}

#[cfg(disabled)]
fn enums() {
    // Enums are types which have a few definite values
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let my_direction = Direction::North;

    // Enums can be used in match statements
    match my_direction {
        Direction::North => println!("We are heading north!"),
        Direction::South => println!("We are heading south!"),
        Direction::East => println!("We are heading east!"),
        Direction::West => println!("We are heading west!"),
    }

    // Enums can also hold data
    enum Shape {
        Circle(f64),         // holds a radius
        Rectangle(f64, f64), // holds width and height
    }

    let my_shape = Shape::Circle(5.0);

    match my_shape {
        Shape::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape::Rectangle(width, height) => {
            println!("Rectangle of size: {} x {}", width, height)
        }
    }
}

// #[cfg(disabled)]
fn pattern_matching() {
    let x = Some(5);

    // Match can also be used for destructuring and pattern matching
    match x {
        Some(i) if i % 2 == 0 => println!("Even number: {}", i),
        Some(i) => println!("Odd number: {}", i),
        None => println!("No value found!"),
    }
}

#[cfg(disabled)]
fn error_handling() {
    // Rust's preferred way uses Result and Option for error handling instead of exceptions

    fn division(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
        if divisor == 0.0 {
            Err("Cannot divide by zero!")
        } else {
            Ok(dividend / divisor)
        }
    }

    match division(5.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    // Another common pattern is to use `unwrap` or `expect`
    // However, be cautious since they can cause the program to panic
    let value = division(10.0, 2.0).unwrap();
    println!("Value is: {}", value);
}

#[cfg(disabled)]
fn clean_error_handling() -> Result<(), &'static str> {
    fn load_value(key: i32) -> Result<&'static str, &'static str> {
        match key {
            ..=-1 | 18.. => Err("Key out of range"),
            _ => Ok("Insert value here"),
        }
    }
    let s = load_value(12)?;
    let s = load_value(20)?;
    Ok(())
}

#[cfg(disabled)]
fn panicking() {
    // Panics are used to indicate that something went very very wrong
    // They can be caused by calling `panic!` or by a failed assertion
    let number = 5;
    if number != 5 {
        panic!("wtf!!");
    }

    assert_eq!(number, 5);
    assert!(number == 5);

    // Many other functions and macros cause panics too
    if 1 == 1 {
        unimplemented!();
        // Or
        todo!();
    }

    if 1 != 1 {
        unreachable!();
    }

    let num_maybe = Some(5);
    let num = num_maybe.unwrap(); // Panics if num_maybe is None
    let num = num_maybe.expect("oh no"); // Panics with the given message if num_maybe is None

    let vec = vec![1, 2, 3];
    let a = vec[99]; // Panics if the index is out of bounds
}

#[cfg(disabled)]
fn zero_cost_abstractions() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Using a basic manual loop
    let mut sum = 0;
    for num in numbers {
        if num % 2 == 0 {
            sum += num * num; // Squaring even numbers
        }
    }

    println!("Sum (without iterator): {}", sum);

    // Using iterators, compiles to the exact same code, sometimes even faster
    let sum: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();

    println!("Sum (with iterator): {}", sum);
}

#[cfg(disabled)]
fn destructuring() {
    let value = Some(42);

    if let Some(x) = value {
        println!("x is {}", x);
    } else {
        println!("value is None");
    }

    // You can also let...else
    let Some(x) = value else {
        return;
    };
    println!("x is {}", x);

    enum MyErrors {
        Foo(String),
        Bar(i32),
        Baz,
    }

    let result: Result<String, MyErrors> = Err(MyErrors::Foo("foo".to_string()));

    match result {
        Ok(x) => println!("x is {}", x),

        Err(MyErrors::Foo(x)) => println!("x is {}", x),
        Err(MyErrors::Bar(x)) => println!("x is {}", x),
        Err(MyErrors::Baz) => println!("x is {}", x),
    }
}

#[cfg(disabled)]
fn borrows() {
    let s1 = String::from("hello");
    let s2 = &s1; // s2 is a borrow to s1

    println!("s1: {}", s1); // This is valid, as s1 is still the owner
    println!("s2: {}", s2); // This prints the value that s2 refers to

    // Mutable borrows
    let mut s1 = String::from("world");
    let s2 = &mut s1; // s4 is a mutable borrow

    s1.push_str(","); // This is an error! You can't use s3 while there's a mutable borrow
    s2.push_str("!!"); // Modifying via s4
    println!("s1: {}", s1); // Prints "world!!"
}

#[cfg(disabled)]
fn moves() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2. s1 can no longer be used!

    // println!("s1: {}", s1); // This would be an error, as s1 is no longer valid
    println!("s2: {}", s2); // This is valid

    // To get back ownership, you would have to clone or return ownership
    let s3 = s2.clone(); // This clones the data, so s2 and s3 are two different strings with the same value
}

#[cfg(disabled)]
fn lifetimes_and_borrowing() {
    // Lifetimes ensure references are valid as long as they are used
    // Borrowing allows data to be accessed without taking ownership

    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }

    let string1 = String::from("long string");
    let string2 = "short";

    let result = longest(&string1, string2);

    println!("The longest string is: {}", result);

    // References help make sure data isn't used after it's freed
    let result: &str;

    let string1 = String::from("short");
    {
        let string2 = String::from("long string");

        result = longest(&string1, &string2);
        // string2 is freed here
    }

    println!("The longest string is: {}", result); // Error
}

#[cfg(disabled)]
struct Book {
    title: String,
    author: String,
}

// Implementing the Drop trait for Book
// This adds extra logic when the struct is dropped
#[cfg(disabled)]
impl std::ops::Drop for Book {
    fn drop(&mut self) {
        println!("Dropping the book '{}'", self.title);
    }
}

#[cfg(disabled)]
fn create_and_drop_book() {
    let rust_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
    };

    // At the end of this function, rust_book goes out of scope.
    // Rust will automatically call the `drop` method we defined,
    // and we will see the "Dropping the book..." message.
}

#[cfg(disabled)]
fn manual_drop() {
    let async_book = Book {
        title: String::from("Asynchronous Programming in Rust"),
        author: String::from("Various Authors"),
    };

    // Explicitly dropping the book early
    drop(async_book);

    // Once we drop it manually, we can't use it anymore
    println!("{}", async_book.title); // This would be an error!

    // Since it's already manually dropped, Rust won't call the `drop` method again
    // at the end of the scope.
}

#[cfg(disabled)]
fn closures() {
    // Inline functions are called closures

    // With curly brackets
    let op = |a, b| {
        println!("Adding {} and {}", a, b);
        a + b
    };
    // Without curly brackets
    let op = |a, b| a + b;

    let x = op(2, 3);
}

fn traits() {
    trait Signed {
        fn is_negative(&self) -> bool;
        fn is_non_negative(&self) -> bool {
            !self.is_negative()
        }
    }

    impl Signed for i32 {
        fn is_negative(&self) -> bool {
            *self < 0
        }
    }
}

fn generic_read() {
    use std::io::{Read, Result, Cursor, Error, ErrorKind};

    let data = Cursor::new("#!/bin/sh");

    enum FileType {
        Shell,
        Png,
        Unknown,
    }


    fn exec<T: Read>(mut file: &mut T) -> Result<FileType> {
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf)?;

        if buf.starts_with(b"#!") {
            Ok(FileType::Shell)
        } else if buf.starts_with(b"PNG") {
            Ok(FileType::Png)
        } else {
            Ok(FileType::Unknown)
        }
    }
}

fn unsafe_code() {
    let x: u32 = 0xDEADBEEF;
    let y = &x as *const u32;
    let z = unsafe { *y };
    println!("{x:#X}");
}
