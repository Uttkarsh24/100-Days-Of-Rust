// Fix the code so that it compiles.

#[derive(Debug)]
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
    let item = ShopItem::new("Pants".to_string(), 450);
    if item.in_stock() {
        println!("{} remaining: {}", item.name, item.quantity);
    } else {
        println!("{} not in stock", item.name);
    }
}