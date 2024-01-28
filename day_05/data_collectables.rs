fn main() {
    // arrays
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    let mut bytes = [0; 5];

    bytes[0] = 1;

    let first = days[0];

    println!("{} and {}.", first, bytes[0]);

    // vectors
    let three_nums = vec![15, 3, 20];
    let zeroes = vec![0; 5];
    println!("Three Nums: {:?} and Zeroes: {:?}.", three_nums, zeroes);

    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Orange");
    println!("Fruits: {:?}.", fruit);

    println!("Pop off: {:?}.", fruit.pop());
    println!("Fruit: {:?}.", fruit);

    // hashmaps
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Orange"), String::from("Orange"));
    reviews.insert(String::from("Banana"), String::from("Yellow"));
    reviews.insert(String::from("Apple"), String::from("Red"));

    let fruit: &str = "Apple";
    println!("Color for \'{}\' : {:?}.", fruit, reviews.get(fruit));

    let remove_fruit: &str = "Banana";
    reviews.remove(remove_fruit);
    println!("\'{}\' removed.", remove_fruit);
}