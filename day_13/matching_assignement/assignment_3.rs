// Fix the code so that it compiles.

enum Operation {
    Add(u8, u8),
    Mul(u8, u8),
    Subtract { first: u8, second: u8 },
    Divide { divident: u8, divisor: u8 },
}

impl Operation {
    fn result(&self) -> u8 {
        match self {
            Self::Add(a, b) => a + b, // notice Self can be used instead of Operation
            Self::Subtract { first, second } => first - second,
            Self::Mul(first, second) => first*second,
            Self::Divide { divident, divisor } => divident / divisor,
        }
    }
}

fn main() {
    let user_operation = Operation::Subtract {
        first: 75,
        second: 20,
    };
    println!("Result: {:?}", user_operation.result());
}