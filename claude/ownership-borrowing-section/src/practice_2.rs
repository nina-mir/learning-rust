// Create a function that borrows a string slice and counts the number of vowels:
pub fn count_vowels(s: &str) -> u32 {
    // Your code here
    // My 1st solution (not rust idiomatic)
    // let bytes = s.as_bytes();
    
    // let mut count:u32 = 0;

    // for &item in bytes.iter() {
    //     if item == b'a' || item == b'i' || item == b'u' || item == b'e' || item == b'o' {
    //         count = count + 1;
    //     }
    // }
    // count
    // 2nd solution (rust idiomatic) Claude suggestion
    s.chars()
        .filter(|&c| "aeiouAEIOU".contains(c))
        .count().try_into().unwrap()
}


pub fn run(){
    println!("{}", count_vowels("aeiou"))
}