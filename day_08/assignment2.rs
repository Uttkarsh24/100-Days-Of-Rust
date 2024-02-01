fn main()
{
    let mut my_str = String::from("Example");
    let mut temp;
    while my_str.len() > 0 {
        temp = my_str.clone();
        println!("Length of temporary string is: {}", temp.len());
        my_str.pop();
    }
}