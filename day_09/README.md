# Borrowing In Rust - Day 9

This is the ninth day you have to Take this challenge and start your #100daysofrust journey.

## Task 

- Fork the repository
- Now Using The [Resource](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) - Learn About `borrowing`.
- Now Create a file name - `borrow.rs` and add a `main` function in it.

## Assignments

### Assignment 1

```rs
// Fix the code so that it compiles.

fn main() {
    let mut str1 = String::from("modifiable");
    let str2 = String::from("fixed string");
    let mut str_ptr: &String;
    str_ptr = str1;
    println!("ptr currently points to {str_ptr}");
    str_ptr = str2;
    println!("ptr currently points to {str_ptr}");
    str1.push_str(" string");
    str_ptr = str1;
    println!("ptr currently points to {str_ptr}");
}
```
- Solution - [assignment_1.rs](./borrow_assignment/assignment_1.rs)

### Assignment 2

```rs
// Fix the code so that it compiles.

fn main() {
    let mut s = String::from("Hello, ");
    let s_ref = &s;
    change_string(s_ref);
    println!("{s_ref}");
}

fn change_string(s: &String) {
    s.push_str(" world!");
}
```
- Solution - [assignment_2.rs](./borrow_assignment/assignment_2.rs)

### Assignment 3

```rs
// Fix the code so that it compiles.

fn main() {
    let str1 = String::from("Rust");
    let str2 = String::from("Golang");
    let ref1 = &mut str1;
    let mut ref2 = &mut str2;

    println!("First string: {ref1}");
    println!("Second string: {ref2}");
    ref1.push('🦀');
    ref2.push('🦫');
    println!("Modified first string: {ref1}");
    println!("Modified second string: {ref2}");
    // only one mutable reference allowed at a time, ref1 is no longer valid
    ref2 = &mut str1;
    ref2.pop();
    println!("Original first string: {ref2}");
}
```
- Solution - [assignment_3.rs](./borrow_assignment/assignment_3.rs)

### Assignment 4

```rs
// Complete the function signature to make the code compile.

fn main() {
    let mut s1 = String::from("this is ");
    let s2 = String::from("an example sentence");
    concat(&mut s1, &s2);
    println!("{s1}")
}

fn concat(s1, s2) {
    for ch in s2.chars() {
        s1.push(ch);
    }
}
```
- Solution - [assignment_4.rs](./borrow_assignment/assignment_4.rs)
