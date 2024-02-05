# Implementation blocks in Rust - Day 12

This is the twelfth day you have to Take this challenge and start your #100daysofrust journey.

## Task

- Fork the repository
- Now Using The [Resource](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) - Learn About `Implementation blocks`.
- Now Create a file name - `implementation_blocks.rs` and add a `main` function in it.

## Assignments

### Assignment 1

```rs
// Complete the method signatures by providing appropriate arguments.

struct Student {
    first_name: String,
    last_name: String,
    roll_no: u16,
}

impl Student {
    fn get_name() -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_roll_no(, new_roll_no: u16) {
        self.roll_no = new_roll_no;
    }
    fn convert_to_string() -> String { // should take ownership
        format!(
            "Name: {} {}, Roll no: {}",
            self.first_name, self.last_name, self.roll_no
        )
    }
}

fn main() {
    let mut student = Student {
        first_name: "Harry".to_string(),
        last_name: "Potter".to_string(),
        roll_no: 42,
    };
    println!("Student is: {}", student.get_name());
    student.set_roll_no(50);
    let student_details = student.convert_to_string();
    println!("{student_details}");
}
```

- Solution - [assignment_1.rs](./implementation_assignement/assignement_1.rs)

### Assignment 2

```rs
// Fix the code so that it compiles.

struct ShopItem {
    name: String,
    quantity: u32,
}

impl ShopItem {
    fn new(name: String, quantity: u32) -> ShopItem {
        ShopItem { name, quantity }
    }
    fn in_stock(&self) -> bool {
        self.quantity > 0
    }
}

fn main() {
    let item = ShopItem.new("Pants".to_string(), 450);
    if item.in_stock() {
        println!("{} remaining: {}", item.name, item.quantity);
    } else {
        println!("{} not in stock", item.name);
    }
}
```

- Solution - [assignment_2.rs](./implementation_assignement/assignement_2.rs)
