fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The lenght of '{}' is {}", s1, len);

    // We cannot change inmutable references
    // let s2 = String::from("hello");
    // change(&s2);

    // Mutable references
    let mut s3 = String::from("hello");
    change(&mut s3);
    println!("{}", s3);

    // Mutable references can have only one mutable reference to a particular piece of data at a time.
    // NOT VALID
    /// let r1 = &mut s3;
    /// let r2 = &mut s3;

    {
        let _r1 = &mut s3;
    }
    let _r2 = &mut s3;

    // The compiler guarantees that references will never be dangling reference
    // (a location in memory that may have been given to someone else)
    //let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
