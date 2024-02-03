fn main() {
    //string slices
    let s = String::from("hello world");

    let hello = &s[0..5]; //you can also drop the 0 if you want to start at zero index
    let world = &s[6..11];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; //you can drop the len if you want to end at last index

    //general slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
