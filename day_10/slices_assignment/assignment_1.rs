fn find_substr_pos(text:&str, substr:&str) -> usize { // both parameters should have same data type
    if text.len() < substr.len() {
        return text.len();
    }
    let len = substr.len();
    for start in 0..text.len() - len + 1 {
        if substr == &text[start..start+len] { // what will be the correct range?
            return start;
        }
    }
    text.len()
}
fn main() {
    //exercise 1
    let text = String::from("Today is a very warm and sunny day.");
    let words = ["very", "arm", "say", "sun", "dew"];
    let mut pos;

    println!("Text: {text}");
    for word in words {
        pos = find_substr_pos(&text, word);
        if pos == text.len() {
            println!("{word} is not present in text");
        } else {
            println!("{word} present at index {pos}");
        }
    }
}