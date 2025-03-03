// Create a function that borrows a string slice and counts the number of vowels:
pub fn count_vowels(s: &str) -> u32 {
    // Your code here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'a' {
            return i.try_into().unwrap();
        }
    }


    
    23
}


pub fn run(){
    println!("{}", count_vowels("nina"))
}