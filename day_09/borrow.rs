// here we have to pass the reference in order to make the string workable in the main function
fn takes_ownership(s: &String) {
    println!("{}", s);
}

// gives the ownership back to the string by returning the value
fn gives_ownership() -> String {
    let some_string = String::from("Hello World");

    some_string
}

// takes the ownership and gives it back
fn takes_and_gives_back(str: String) -> String {
    str
}

// passing mutable string in function
fn change_string(some_string: &mut String) {
    some_string.push_str(" World");

    println!("{}", some_string);
}

fn main() {
    // refernce of values in case of passing to a function
    let s: String = String::from("Hello");
    takes_ownership(&s);
    println!("{}",s);

    let str1: String = gives_ownership();
    println!("{}", str1);

    let str2: String = takes_and_gives_back(str1);
    println!("{}", str2);

    // passing mutable string to function
    let mut st: String = String::from("Hello World");
    change_string(&mut st);

    // for a variable you can have only one mutable reference and unlimited immutable reference
}