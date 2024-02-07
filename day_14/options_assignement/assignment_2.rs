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
        Some(ele) => println!("Last element: {ele}"),
        None => println!("Empty array"),
    }
}