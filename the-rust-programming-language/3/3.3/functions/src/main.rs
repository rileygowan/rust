fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Statement
fn main() {
    let y = 6;
}

// Expression
fn main() {
    let x = 5;

    let y = {
        // This is an expression
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// Return values
fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn five() -> u8 {
    5
}

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
