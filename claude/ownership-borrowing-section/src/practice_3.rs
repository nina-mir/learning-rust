// Implement a function that demonstrates the difference between copying and moving:

pub fn ownership_example() {
    // Your code here
    let erstes = String::from("hallo!");
    
    let kopie = erstes.clone();
    println!("erstes variable of {} is copied to kopie variable of\t{}", erstes, kopie);

    let zweites = erstes ;
    println!("erstes is no longer owning anything but zweites owns it:\t{}", zweites);

}

pub fn run(){
    ownership_example();
}

