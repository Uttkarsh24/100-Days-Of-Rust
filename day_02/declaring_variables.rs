fn main() {
    // Declare a variable
    let a_number;

    // Declare a second variable and bind the value
    let a_word = "Ten";

    // Bind a value to the first variable
    a_number = 10;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    //mutable variable
    let mut a_num = 15;
    println!("the number is {}", a_num);

    a_num = 10;
    println!("the number is {}", a_num);

    //variable shadowing
    let shadow_num = 69;

    //second variable shadowing
    let shadow_num = shadow_num + 5;

    //third variable shadowing
    let shadow_num = shadow_num * 2;

    println!("the number is {}", shadow_num);
}
