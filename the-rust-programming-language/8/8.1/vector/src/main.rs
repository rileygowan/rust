// fn main() {
//     let v: Vec<i32> = Vec::new();
//     let v = vec![1, 2, 3];

//     let mut v = Vec::new();

//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);

//     {
//         let v = vec![1, 2, 3, 4];

//         // do stuff with v
//     } // <- v goes out of scope and is freed here
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {}", third);

//     match v.get(2) {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("There is no third element."),
//     }
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     // let does_not_exist = &v[100];
//     let does_not_exist = v.get(2);

//     match does_not_exist {
//         Some(&number) => println!("{}", number),
//         None => println!("Not a damn thing"),
//     }

//     // println!("{:?}", does_not_exist)
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0];

//     v.push(6);

//     println!("The first element is: {}", first);
// }

// fn main() {
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }
// }

fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v)
}