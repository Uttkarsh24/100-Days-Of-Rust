// Complete the function signature to make the code compile.

fn main() {
    let mut s1 = String::from("this is ");
    let s2 = String::from("an example sentence");
    concat(&mut s1, &s2);
    println!("{s1}")
}

fn concat(s1: &mut String, s2: &String) {
    for ch in s2.chars() {
        s1.push(ch);
    }
}