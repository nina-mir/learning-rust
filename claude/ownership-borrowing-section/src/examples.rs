pub fn run() {
    // Example 1: Basic ownership
    let s1 = String::from("hello");
    let s2 = s1;  // s1's ownership moves to s2
    println!("{}", s2);  // This would fail - value moved

    // Example 2: Cloning instead of moving
    let s3 = String::from("hello");
    let s4 = s3.clone();  // Creates a deep copy
    println!("s3: {}, s4: {}", s3, s4);  // This works!

    // Borrowing
    let mut s = String::from("hello");

    // Example 3: Immutable borrow
    let r1 = &s;
    let r2 = &s;
    println!("{} and {} and original is {}", r1, r2, s);
    
    // Example 4: Mutable borrow
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
    println!("And original is {}", s); // absolutely changed and can be used now!

}

