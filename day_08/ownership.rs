fn main()
{
    //variable scope
    {                      // s is not valid here, as it s not yet declared
        let s = "hello";   // s is declared and valif

        // do whatever with s
    }                      // this scope is now over and rust drops the value of s

    //the String type

    //note:do not confuse string and String they are not the same
    let mut x = String::from("hello");

    x.push_str(", world!"); // adds ", world" to x

    println!("{}", x);

    //copy and clone concept in rust
    let x=69;
    ley y=x;
    println!("x={} , y={}",x,y); // since integers are simple values with a fixed size they are pushed onto the stack
    //same is the case: boolean,character float types and tuples which contain the same.

    let x1 = String::from("Hello");
    let x2 = x1;
    println!("{}",x1);//this shows an error as the String x1 is moved to x2 instead of getting copied.
    //this happens because after line 24 rust considers x1 as no longer valid.

    //to fix this use the .clone() method
    let x1 = String::from("hello");
    let x2 = x1.clone();

    println!("x1 = {}, x2 = {}", x1, x2);
}