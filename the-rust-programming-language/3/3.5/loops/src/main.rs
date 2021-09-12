// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let array = [0, 1, 2, 3, 4];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", array[index]);

//         index += 1
//     }
// }

// fn main() {
//     let a = [100, 12, 23, 34, 6];

//     for element in a.iter() {
//         println!("The value is: {}", element);
//     }
// }

fn main() {
    for element in (1..4).rev() {
        println!("{}", element);
    }

    println!("LIFTOFF");
}
