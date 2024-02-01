# Ownership In Rust - Day 8

This is the eighth day you have to Take this challenge and start your #100daysofrust journey.

## Task 

- Fork the repository
- Now Using The [Resource](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) - Learn About `ownership`.
- Now Create a file name - `ownership.rs` and `ownership_in_function.rs` - and add a `main` function in it.

## Assignments

### Assignment 1

```rs
// TODO - Something's missing. Fix the code so that it compiles.

fn main() {
    let s1 = String::from("Rust");
    let mut s2 = s1;
    s2.push_str(" is an awesome language");
    println!("String:\"{s1}\" is a substring of \"{s2}\"");
}
```
- Solution - [assignment_1.rs](./assignment_1.rs)

### Assignment 2

```rs
// TODO - Fix the code so that it compiles. Modify only one statement.

fn main() {
    let mut my_str = String::from("Example");
    let mut temp;
    while my_str.len() > 0 {
        temp = my_str;
        println!("Length of temporary string is: {}", temp.len());
        my_str.pop();
    }
}
```
- Solution - [assignment_2.rs](./assignment_2.rs)

### Assignment 3

```rs
// Fix the code so that it compiles.

fn main() {
    let my_string = String::from("I love rust bootcamp 💕");
    let occurence_count = count_occurences(my_string, 'o');
    println!("The number of times 'o' apprears in \"{my_string}\" = {occurence_count}");
}

// this function counts the number of times a letter appears in a text
fn count_occurences(text: String, letter: char) -> u32 {
    let mut res = 0;
    for ch in text.chars() {
        if ch == letter {
            res += 1;
        }
    }
    res
}
```
- Solution - [assignment_3.rs](./assignment_3.rs)
