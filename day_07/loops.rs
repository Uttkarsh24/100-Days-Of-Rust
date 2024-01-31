fn main() {
    // loop
    loop {
        println!("OK!");
        break;
    }

    let mut counter = 1;
    let stop_loop = loop {
        counter *= 2;
        if counter > 10 {
            break counter;
        }
    };
    println!("Break the loop at counter = {}.", stop_loop);

    // while loop
    counter = 1;
    while counter < 5 {
        println!("Looping....");
        counter += 1;
    }

    // looping through values
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("{}", bird);
    }

    for number in 0..5 {
        println!("{}", number * 2);
    }
}
