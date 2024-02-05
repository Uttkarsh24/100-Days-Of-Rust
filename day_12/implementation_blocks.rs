#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation blocks
impl Rectangle {
    fn area(&self) -> u32  {
        self.height*self.width
    }  

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    let square = Rectangle::square(32);

    println!("{:#?}", rect);
    println!("Area - {}", rect.area());
    println!("Check - {}", rect.can_hold(&rect1));

    println!("{:#?}", square);
}
