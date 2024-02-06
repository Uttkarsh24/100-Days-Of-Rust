fn main() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("3"),
        _ => (),
    }

    // using if-let for the above
    if let Some(3) = some_value {
        println!("Three");
    }
}