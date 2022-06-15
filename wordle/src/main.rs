use rand::prelude::*;
use std::num::ParseIntError;

//     colors
//         .iter()
//         .map(|c| c.to_string())
//         .collect::<Vec<String>>()
//         .join(" ")
// }

fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    let mut colors: [&str; 4] = ["⬜", "⬜", "⬜", "⬜"];

    let mut secret = Vec::from(secret);

    for ((index, g), s) in guess.iter().enumerate().zip(secret.iter_mut()) {
        if g == s {
            colors[index] = "🟩";
            *s = i32::MAX;
        }
    }

    for v in guess.iter() {
        if let Some(pos) = secret.iter().position(|other| v == other) {
            colors[pos] = "🟨";
        }
    }

    colors.join(" ")
}
fn main() {
    let num_digits = 4;
    let mut rng = thread_rng();
    let secret: Vec<_> = (0..num_digits).map(|_| rng.gen_range(0..9)).collect();
    let stdin = std::io::stdin();
    let mut buf = String::new();

    loop {
        buf.clear();
        print!("guess: ");
        stdin.read_line(&mut buf).unwrap();
        let guess: Result<Vec<i32>, ParseIntError> =
            buf.trim().split(' ').map(|s| s.parse()).collect();
        match guess {
            Ok(guess) => {
                let squares = calc_green_and_yellow(&guess, &secret);
                println!("{:?} got {:?}", guess, squares);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

#[test]
fn test_green_and_yellow() {
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 🟩".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]),
        "🟨 🟨 🟨 🟨".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]),
        "🟩 🟩 🟩 ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]),
        "🟩 ⬜ ⬜ ⬜".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]),
        "🟨 🟩 🟩 🟨".to_string()
    );
    assert_eq!(
        calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]),
        "🟨 ⬜ ⬜ 🟨".to_string()
    );
}
