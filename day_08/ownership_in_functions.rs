fn main() {
    let eg = String::from("hello");  // eg comes into scope

    takes_ownership(eg);             // eg's value moves into the function
                                    // ... and value of s is droppped by rust

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move to function but because it's i32 the value is not dropped

} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called tp free the heap memory


fn makes_copy(some_integer: i32) { 
    println!("{}", some_integer);
} //some_integer goes out of scope 