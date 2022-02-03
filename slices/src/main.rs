fn main() {

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The size of '{}' is {}", s2, len);

    let s3 = String::from("hello world");
    let word = first_word(&s3);
    //s3.clear();
    println!("the first word is: {}", word);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}