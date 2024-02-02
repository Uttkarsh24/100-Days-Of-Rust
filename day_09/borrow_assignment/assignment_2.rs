// Fix the code so that it compiles.

fn main() {
    let s = String::from("Hello, ");
    let mut s_ref = s;
    change_string(&mut s_ref);
    println!("{s_ref}");
}

fn change_string(s: &mut String) {
    s.push_str(" world!");
}