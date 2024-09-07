#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Value of 1 penny = {}c", value_in_cents(Coin::Penny));
    println!("Value of 1 nickel = {}c", value_in_cents(Coin::Nickel));
    println!("Value of 1 dime = {}c", value_in_cents(Coin::Dime));
    println!("Value of 1 quarter = {}c", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}