// Fix the code so that it compiles.

// USD coin types
// cent values: penny:1, nickel:5, dime: 10, quarter:25
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let piggy_bank = [Coin::Nickel, Coin::Penny, Coin::Dime, Coin::Penny];
    let mut my_savings = 0;
    for coin in piggy_bank {
        my_savings += value_in_cents(coin);
    }
    println!("My savings: {my_savings} cents");
}