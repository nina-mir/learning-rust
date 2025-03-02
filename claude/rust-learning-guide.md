# Rust Learning Series: A 5-Part Guide for Beginners

## Table of Contents
1. [Part 1: Ownership and Borrowing](#part-1-ownership-and-borrowing)
2. [Part 2: Type System and Pattern Matching](#part-2-type-system-and-pattern-matching)
3. [Part 3: Understanding Lifetimes](#part-3-understanding-lifetimes)
4. [Part 4: Structs and Implementations](#part-4-structs-and-implementations)
5. [Part 5: Error Handling in Rust](#part-5-error-handling-in-rust)

## Part 1: Ownership and Borrowing

### Learning Objectives
- Understand Rust's ownership rules
- Learn how to borrow values
- Master working with references
- Comprehend move semantics

### Key Concepts

#### Ownership Rules
1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped
3. Ownership can be transferred (moved)

```rust
fn main() {
    // Example 1: Basic ownership
    let s1 = String::from("hello");
    let s2 = s1;  // s1's ownership moves to s2
    // println!("{}", s1);  // This would fail - value moved

    // Example 2: Cloning instead of moving
    let s3 = String::from("hello");
    let s4 = s3.clone();  // Creates a deep copy
    println!("s3: {}, s4: {}", s3, s4);  // This works!
}
```

#### Borrowing
```rust
fn main() {
    let mut s = String::from("hello");
    
    // Immutable borrow
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // Mutable borrow
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
}
```

### Practice Problems

1. Write a function that takes ownership of a string, modifies it, and returns ownership back:
```rust
fn modify_string(s: String) -> String {
    // Your code here
}
```

2. Create a function that borrows a string slice and counts the number of vowels:
```rust
fn count_vowels(s: &str) -> u32 {
    // Your code here
}
```

3. Implement a function that demonstrates the difference between copying and moving:
```rust
fn ownership_example() {
    // Your code here
}
```

### Additional Resources
1. [Rust Book Chapter on Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
2. [Rust By Example - Ownership and Moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)
3. [Rust Reference - Ownership](https://doc.rust-lang.org/reference/ownership.html)

## Part 2: Type System and Pattern Matching

### Learning Objectives
- Master Rust's basic types
- Understand compound types
- Learn pattern matching with match expressions
- Work with Option and Result types

### Key Concepts

#### Basic Types
```rust
fn main() {
    // Integers
    let x: i32 = 42;
    let y: u64 = 100;
    
    // Floating-point
    let pi: f64 = 3.14159;
    
    // Boolean
    let is_rust_fun: bool = true;
    
    // Character
    let letter: char = 'A';
    
    // Arrays and Tuples
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let tuple: (i32, f64, char) = (42, 3.14, 'A');
}
```

#### Pattern Matching
```rust
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

fn handle_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
    }
}
```

### Practice Problems

1. Create an enum representing different shapes and implement area calculation:
```rust
enum Shape {
    // Your code here
}

fn calculate_area(shape: Shape) -> f64 {
    // Your code here
}
```

2. Implement a function that uses pattern matching to handle Option types:
```rust
fn process_number(num: Option<i32>) -> i32 {
    // Your code here
}
```

3. Create a function that works with different numeric types using generics:
```rust
fn sum<T>(a: T, b: T) -> T {
    // Your code here
}
```

### Additional Resources
1. [Rust Book - Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
2. [Rust By Example - Types](https://doc.rust-lang.org/rust-by-example/types.html)
3. [Rust Book - Pattern Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)

## Part 3: Understanding Lifetimes

### Learning Objectives
- Understand what lifetimes are and why they're needed
- Learn lifetime annotation syntax
- Master common lifetime patterns
- Work with lifetime constraints

### Key Concepts

#### Basic Lifetime Annotations
```rust
// Function with lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct with lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

#### Lifetime Elision Rules
```rust
// These functions have implicit lifetime annotations
fn first_word(s: &str) -> &str {
    // Implementation
}

fn get_str() -> &'static str {
    "Hello, world!"
}
```

### Practice Problems

1. Implement a function that returns the longer of two string slices:
```rust
fn longer_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // Your code here
}
```

2. Create a struct that holds references and implement methods for it:
```rust
struct BookReference<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> BookReference<'a> {
    // Implement methods here
}
```

3. Write a function that demonstrates multiple lifetime parameters:
```rust
fn complex_lifetime_example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    // Your code here
}
```

### Additional Resources
1. [Rust Book - Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
2. [Rust By Example - Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
3. [Rust Reference - Lifetime Parameters](https://doc.rust-lang.org/reference/lifetime-params.html)

## Part 4: Structs and Implementations

### Learning Objectives
- Define and use structs
- Implement methods for structs
- Work with traits
- Understand derive attributes

### Key Concepts

#### Struct Definition and Implementation
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Constructor
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

#### Traits
```rust
trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Rectangle {
    fn summarize(&self) -> String {
        format!("Rectangle: {}x{}", self.width, self.height)
    }
}
```

### Practice Problems

1. Create a struct representing a bank account with methods for deposit and withdrawal:
```rust
struct BankAccount {
    // Your code here
}

impl BankAccount {
    // Implement methods here
}
```

2. Implement a trait for different geometric shapes:
```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Implement for different shapes
```

3. Create a generic struct with implementations:
```rust
struct Pair<T> {
    // Your code here
}

impl<T> Pair<T> {
    // Implement methods here
}
```

### Additional Resources
1. [Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
2. [Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
3. [Rust By Example - Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

## Part 5: Error Handling in Rust

### Learning Objectives
- Understand panic! vs Result
- Master error propagation
- Learn custom error types
- Handle multiple error types

### Key Concepts

#### Result Type and Error Handling
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

#### Custom Error Types
```rust
#[derive(Debug)]
enum CustomError {
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> CustomError {
        CustomError::IoError(err)
    }
}
```

### Practice Problems

1. Implement a function that reads a file and returns the contents or an error:
```rust
fn read_and_process_file(path: &str) -> Result<String, std::io::Error> {
    // Your code here
}
```

2. Create a custom error type and implement error handling:
```rust
enum MyError {
    // Define error variants here
}

fn process_data() -> Result<(), MyError> {
    // Your code here
}
```

3. Write a function that demonstrates error propagation with the ? operator:
```rust
fn complex_operation() -> Result<String, Box<dyn Error>> {
    // Your code here
}
```

### Additional Resources
1. [Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
2. [Rust By Example - Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
3. [Error Handling Best Practices Blog](https://blog.burntsushi.net/rust-error-handling/)

## Conclusion

This learning series covers the fundamental concepts you need to get started with Rust. Remember to:
- Practice each concept with the provided exercises
- Refer to the official documentation
- Join the Rust community on [users.rust-lang.org](https://users.rust-lang.org/)
- Use the Rust Playground to experiment with code

Good luck with your Rust learning journey!
