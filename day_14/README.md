# Option Enum - Day 14
 
This is the fourteenth day you have to Take this challenge and start your #100daysofrust journey.
 
## Task
 
- Fork the repository
- Now Using The [Resource](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) - Learn About `Option` Enum.
- Now Create a file name - `option_enum.rs` - then add a `main` function in it.
 
## Assignments
 
### Assignment 1
 
```rs
// Fix the code so that it compiles.
 
struct Point {
    x: i32,
    y: i32,
}
 
fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
 
    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
```
 
- Solution - [assignment_1.rs](./option_enum_assignement/assignement_1.rs)
 
### Assignment 2
 
```rs
// Fix the code so that it compiles.
 
fn last_element(nums: &[i32]) -> Option<i32> {
    if nums.len() > 0 {
        Some(nums[nums.len() - 1])
    } else {
        None
    }
}
 
fn main() {
    let my_nums = [1, 1, 2, 3, 5, 8, 13];
    match last_element(&my_nums) {
        Some => println!("Last element: {ele}"),
        None => println!("Empty array"),
    }
}
```
 
- Solution - [assignment_2.rs](./option_enum_assignement/assignement_2.rs)