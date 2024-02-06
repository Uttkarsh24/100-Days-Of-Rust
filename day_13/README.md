# Matching & If-Let - Day 13
 
This is the thirteenth day you have to Take this challenge and start your #100daysofrust journey.
 
## Task
 
- Fork the repository
- Now Using The [Resource](https://doc.rust-lang.org/book/ch06-03-if-let.html) - Learn About `Matching` and `If-Let`.
- Now Create a file name - `match.rs` and `if-let.rs`- then add a `main` function in it.
 
## Assignments - Matching
 
### Assignment 1
 
```rs
// Make the following code compile.
// If you score 50 or less, you fail.
 
fn main() {
    // marks scored out of 100
    let marks = 75u8;
    match marks {
        91..=100 => println!("You performed excellent!"),
        71..=90 => println!("You performed good :)"),
        51..=70 => println!("Your performance was average..."),
        0..=30 => println!("You failed. Better luck next time."),
        101..=u8::MAX => println!("Invalid marks!!!"),
    }
}
```
 
- Solution - [assignment_1.rs](./matching_assignement/assignement_1.rs)
 
### Assignment 2
 
```rs
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
```
 
- Solution - [assignment_2.rs](./matching_assignement/assignement_2.rs)
 
### Assignment 3
 
```rs
// Fix the code so that it compiles.
 
enum Operation {
    Add(u8, u8),
    Mul(u8, u8),
    Subtract { first: u8, second: u8 },
    Divide { divident: u8, divisor: u8 },
}
 
impl Operation {
    fn result(&self) -> u8 {
        match self {
            Self::Add(a, b) => a + b, // notice Self can be used instead of Operation
            Self::Subtract { first, second } => first - second,
            Self::Divide { divident, divisor } => divident / divisor,
        }
    }
}
 
fn main() {
    let user_operation = Operation::Subtract {
        first: 75,
        second: 20,
    };
    println!("Result: {}", user_operation::result());
}
```
 
- Solution - [assignment_3.rs](./matching_assignement/assignement_3.rs)
 
## Assignments - If-Let
 
### Assignment 1
 
```rs
// Fix the code so that it compiles.
 
fn last_element(nums: &[i32]) -> Option<i32> {
    if nums.len() > 0 {
        Some(nums[nums.len() - 1])
    } else {
        None
    }
}
 
fn main() {
    let my_nums = [1, 1, 2, 3, 5, 8, 13];
    match last_element(&my_nums) {
        Some => println!("Last element: {ele}"),
        None => println!("Empty array"),
    }
}
```
 
- Solution - [assignment_1.rs](./if_let_assignement/assignement_1.rs)
 