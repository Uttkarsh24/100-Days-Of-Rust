/*
Format of Option Enum -

enum Option<T> {
    Some(T),
    None,
}
*/

fn main() {
    let some_number = Some(5);
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;

    // extracting value of enum
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum: i8 = x + y.unwrap_or(0);
    println!("{}",sum);
}