/* The format of RESULT Enum:

enum Result<T, E> {
    Ok(T):  // A value T was obtained.
    Err(E): // An error of type E was encountered instead.
}*/

/*In the following example code, there's an implementation for a safe_division function that returns either:

1.A Result value with an Ok variant that carries the result of a successful division.
2/An Err variant that carries a struct DivisionByZeroError, which signals an unsuccessful division.*/

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("{:?}", safe_division(9.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0, 2.0));
}