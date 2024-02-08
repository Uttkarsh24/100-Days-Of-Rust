fn greet(name: &str) -> Result<(), String> {
    if name.len() > 0 {
        println!("Hello {name}!");
        Ok(())
    } else {
        Err("Empty name provided".to_string())
    }
  }
  
  fn main() {
    let name = "Tom";
    if let Err(e) = greet(name) {
        println!("Error: {e}");
    }
  }