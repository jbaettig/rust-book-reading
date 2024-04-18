#[derive(Debug)]
enum UsState {
    Alabama,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) {
    match coin {
        // each of these is called an 'arm'
        Coin::Penny => println!("Penny has {} cents.", 1),
        Coin::Nickel => println!("Nickel has {} cents.", 5),
        Coin::Dime => println!("Dime has {} cents.", 10),
        Coin::Quarter(state) => println!("Quarter has {} cents and is from {:?}.", 25, state),
    }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alabama));
}
