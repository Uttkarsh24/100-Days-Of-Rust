//function for part1
fn count_occurences(text: &String, letter: char) -> u32 {
    let mut res = 0;
    for ch in text.chars() {
        if ch == letter {
            res += 1;
        }
    }
    res
}
//function fot part2
fn get_new_string() -> String {
    let new_string = String::from("I will master rust 🦀 🦀");
    new_string
}
fn main()
{
    //part1
    et my_string = String::from("I love rust bootcamp 💕");
    let occurence_count = count_occurences(&my_string, 'o');
    println!("The number of times 'o' apprears in \"{my_string}\" = {occurence_count}");

    //part2
    let mut str1 = get_new_string();
    println!("Printing through str1: {}", str1);
    let mut str2 = str1;
    println!("Printing through str2: {}", str2);
    str1 = str2;
    println!("Again printing through str1: {}", str1);
    str2 = str1.clone();
    println!("Again printing through str2: {}", str2);
    println!("Printing thourgh both: {}, {}", str1, str2);

} 
