fn main() {
    //unsigned integer for signed use 'i'
    let number: u32 = 14;
    println!("The number is {}.", number);

    //floating types
    let number_64 = 4.0; //64bit integer
    let number_32: f32 = 5.0; //32bit integer

    println!("f64 is {} and f34 is {}", number_64, number_32);

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    //boolean types
    let is_bigger = 1 > 4;
    println!("Is 1>4? {}", is_bigger);

    //the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';

    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}.",
        smiley_face, character_1, string_1, character_2, string_2
    );
}