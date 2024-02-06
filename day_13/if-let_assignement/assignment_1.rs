// Fix the code so that it compiles.

struct User {
    id: u32,
    name: String,
}

fn get_user_name(id: u32) -> Option<String> {
    let database = [
        User {id: 1, name: String::from("Alice")},
        User {id: 2, name: String::from("Bob")},
        User {id: 3, name: String::from("Cindy")}
    ];
    for user in database {
        if user.id == id {
            return Some(user.name)
        }
    }
    None
}

fn main() {
    let user_id = 3;
    if let Some(name) = get_user_name(user_id) {
        println!("User's name: {name}");
    }
}