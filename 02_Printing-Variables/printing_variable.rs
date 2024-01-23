fn main() {
    println!();

    // printing statement
    println!(
        "The first letter of the English alphabet is {} and the last letter is {}.",
        'A', 'Z'
    );

    println!();

    // declaring variables
    let a_number;

    let a_word = "Ten";

    a_number = 10;

    println!("Number is {}.", a_number);
    println!("The word is {}.", a_word);

    println!();

    // mutable variable - !Warning
    let mut a_num = 10;

    a_num = 15;
    println!("Mutable Variable Is - {}.", a_num);

    println!();

    // variable shadowing
    let shadow_num = 1;

    let shadow_num = shadow_num + 5;

    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);
}
