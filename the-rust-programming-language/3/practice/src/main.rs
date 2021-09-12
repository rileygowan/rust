// Convert temperatures between Fahrenheit and Celsius.
fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.) / 1.8
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9. / 5.) + 32.
}
// Generate the nth Fibonacci number.
fn get_nth_fibonacci_number(n: u128) -> u128 {
    if n <= 1 {
        return 1;
    }

    get_nth_fibonacci_number(n - 1) + get_nth_fibonacci_number(n - 2)
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn main() {
    // let celsius = convert_fahrenheit_to_celsius(20.);
    // let fahrenheit = convert_celsius_to_fahrenheit(10.);

    // println!("20 fahrenheit is {} celsius", celsius);
    // println!("10 celsius is {} fahrenheit", fahrenheit);
    let number = 5;
    let result = fibonacci(number);

    println!("The {}th fibonacci number is: {}", number, result)
}

fn the_twelve_days_of_christmas() {
    
}

// [Verse 1]
// On the first day of Christmas, my true love sent to me
// A partridge in a pear tree

// [Verse 2]
// On the second day of Christmas, my true love sent to me
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 3]
// On the third day of Christmas, my true love sent to me
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 4]
// On the fourth day of Christmas, my true love sent to me
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 5]
// On the fifth day of Christmas, my true love sent to me
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 6]
// On the sixth day of Christmas, my true love sent to me
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 7]
// On the seventh day of Christmas, my true love sent to me
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 8]
// On the eighth day of Christmas, my true love sent to me
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 9]
// On the ninth day of Christmas, my true love sent to me
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 10]
// On the tenth day of Christmas, my true love sent to me
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 11]
// On the eleventh day of Christmas, my true love sent to me
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

// [Verse 12]
// On the twelfth day of Christmas, my true love sent to me
// Twelve drummers drumming
// Eleven pipers piping
// Ten lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree
