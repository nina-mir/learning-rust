# An approach for organizing all practice problems within a single project:

## Using Multiple Files with Rust Modules

1. Create one project for each section:

```cargo new ownership_section```

2. Inside the project, create a separate file for each example/practice problem in the src directory:

```
src/
├── main.rs
├── example1.rs
├── practice1.rs
├── practice2.rs
└── practice3.rs
```

In each file, create a public function that contains the example:

```
// In src/practice1.rs
pub fn modify_string_solution(s: String) -> String {
    let mut s = s;
    s.push_str(" - modified");
    s
}

pub fn run() {
    let original = String::from("Hello");
    let modified = modify_string_solution(original);
    println!("Modified string: {}", modified);
}
```

In your ```main.rs```, declare these files as modules and call their run functions:

```
// In src/main.rs
mod example1;
mod practice1;
mod practice2;
mod practice3;

fn main() {
    println!("Running Example 1:");
    example1::run();
    
    println!("\nRunning Practice 1:");
    practice1::run();
    
    println!("\nRunning Practice 2:");
    practice2::run();
    
    println!("\nRunning Practice 3:");
    practice3::run();
}
```

You can choose which example to run by commenting out the other calls:

```
main() {
    // println!("Running Example 1:");
    // example1::run();
    
    println!("\nRunning Practice 1:");
    practice1::run();
    
    // println!("\nRunning Practice 2:");
    // practice2::run();
    
    // println!("\nRunning Practice 3:");
    // practice3::run();
}
```

## Alternative: Using Feature Flags

For more advanced organization:

1. Add feature flags in your Cargo.toml:
```
[features]
example1 = []
practice1 = []
practice2 = []
practice3 = []
```
2. Use conditional compilation in main.rs:

```
mod example1;
mod practice1;
mod practice2;
mod practice3;

fn main() {
    #[cfg(feature = "example1")]
    {
        println!("Running Example 1:");
        example1::run();
    }
    
    #[cfg(feature = "practice1")]
    {
        println!("Running Practice 1:");
        practice1::run();
    }
    
    // And so on...
}
```

3. Run specific examples with:

```cargo run --features practice1```

This approach lets you maintain all examples and practice problems in one project while running them selectively. The module approach is simpler to start with, while feature flags offer more flexibility as you become more familiar with Rust's tooling.