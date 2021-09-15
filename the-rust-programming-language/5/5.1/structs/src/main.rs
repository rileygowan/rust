#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Names(String, String, String);

fn main() {
    let person = Names(
        String::from("Thomas"),
        String::from("Riley"),
        String::from("Gowan"),
    );

    let user = build_user(String::from("t.rileygowan@gmail.com"), person.0.to_lowercase());

    let user2 = User {
        active: false,
        ..user
    };

    println!("{:?}", user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
