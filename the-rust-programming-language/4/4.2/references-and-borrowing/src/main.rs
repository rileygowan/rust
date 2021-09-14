// fn main() {
//     let s1 = String::from("Cool");

//     let length = get_length(&s1);

//     println!("The length of '{}' is {}", s1, length);
// }

// fn get_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    // dangle();
    no_dangle();
}

// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("Hello");

    s
}
