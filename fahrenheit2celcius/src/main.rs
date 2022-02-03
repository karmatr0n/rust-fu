use std::io;

fn main() {
    println!("Provide the temperature in fahrenheit degrees: ");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Read input failed!");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(fahrenheit) => fahrenheit,
        Err(_) => {
            panic!("That wasn't valid input! Temperatures can only be float!");
        }
    };

    let celcious = fahrenheit_to_celcious(fahrenheit);
    println!("{} Farenheit degrees are {} Celcius degress", fahrenheit, celcious);
}

fn fahrenheit_to_celcious(f: f64) -> f64 {
    (f - 32.0) / 1.8
}