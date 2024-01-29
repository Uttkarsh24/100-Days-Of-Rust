//function
fn goodbye() {
    println!("goodbye! my rusty fellows");
}
//funtion with arguements
fn goodbye1(message: &str) {
    println!("\n{}", message);
}

//function with return type
fn divide_by_3(num: u32) -> u32 {
    num / 3
}

fn main() {
    println!("Hello! my rusty fellows");
    goodbye();

    let formal = "formal: goodbye!";
    let informal = "informal: see you later";
    goodbye1(formal);
    goodbye1(informal);
    
    let num: u32 = 69;
    println!("{} divided by 3 = {}", num, divide_by_3(num));
}