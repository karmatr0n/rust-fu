fn main() {
    let _s = "hello"; // Literal string
    let mut s1 = String::from("Hello");
    s1.push_str(", World!");
    println!("{}", s1);
    let s2 = s1;
    println!("{}", s2);
    let s3 = s2.clone();
    println!("{}", s3);
    // When a variable goes out of scope, Rust calls a special function for us. 
    // This function is called drop, and it’s where the author of String can 
    // put the code to return the memory. Rust calls drop automatically at t
    // he closing curly bracket.

    // Passing a variable to a function will move or copy, just as assignment does
    takes_ownership(s3);
    //println!("{}", s3); 
    
    let x = 5;
    makes_copy(x);

    let _s4 = gives_ownership(); // Transfer ownership to the current scope
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);

    //this is too much ceremony and a lot of work for a concept that should be
    let (s7, len) = calculate_length(s6);
    println!("The length of '{}' is {}.", s7, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}