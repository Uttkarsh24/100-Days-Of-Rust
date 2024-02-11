// bringing item to scope

use std::time::SystemTime;

fn main() {
  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
      Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
      Err(_) => panic!("SystemTime before UNIX EPOCH!"),
  }
}