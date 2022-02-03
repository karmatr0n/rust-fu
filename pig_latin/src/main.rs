use std::env;
use regex::Regex;
fn main() {
    let args: Vec<String> = env::args().collect();
    let word: &String = &args[1];
    let re: Regex = Regex::new(r"^(?i)(a|e|i|o|u)").unwrap();
    if re.is_match(word) {
        let pig_latin_word = word.clone() + "yay";
        println!("Starts with a {}", pig_latin_word);
    } else {
        let prefix = &word[1..word.len()];
        let first_char = &word[0..1];
        let pig_latin_word = prefix.to_owned() + first_char + "ay";
        println!("{}", pig_latin_word);
    }
}
