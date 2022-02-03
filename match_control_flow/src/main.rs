enum UsState {
    Alabama,
    Alaska,
    Whashington,
    California,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_incents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter form {:?}!", state);
            25
        }
    }
}
fn main() {
    value_incents(Coin::Quarter(UsState::Alaska));
}
