mod stuff;

pub use crate::stuff::alright;

pub fn main() {
    let cool_stuff = alright::do_cool_stuff();

    println!("{:?}", cool_stuff);
}