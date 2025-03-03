// Write a function that takes ownership of a string, modifies it, and returns ownership back:

pub fn modify_string(s: String) -> String {
    // Your code here
    let mut s = s;
    s.push_str(" -modifiziert");
    s
}


pub fn run(){
    let original = String::from("Pochi is funny!");
    let modified = modify_string(original);
    println!("modified string is {}", modified);
}