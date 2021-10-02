use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
fn main() {
    // let mut numbers = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    // println!("AVERAGE: {}", average(&numbers));
    // println!("MEDIAN: {}", median(&mut numbers));
    // println!("MODE: {}", mode(&numbers));

    // Convert strings to pig latin.
    // The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
    // Keep in mind the details about UTF-8 encoding!
    // let mut input = String::new();
    // loop {
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");
    //     input = input.trim().to_string();
    //     piglatinize(&mut input);
    //     println!("{}", input);
    //     input.clear();
    // }
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();
    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
    for line in stdin.lock().lines() {
        let input = line.expect("error: unable to read user input");
        match Command::from_input(&input) {
            // or_default is just a convenience, does the same as or_insert_with(Vec::default)
            Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
                None => println!("I don't recognize that department!"),
            },
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            }
            Some(Command::Quit) => break,
            // consider using eprintln, which prints to stderr
            None => println!("Input error!"),
        }
    }
    println!("Have a nice day!");
}

enum Command {
    // Using named fields instead of Add(String, String) because dept and name
    // are the same type and could get mixed up.
    Add { dept: String, name: String },
    List(String),
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        // "Slice destructuring / slice pattern matching" for more info
        match words.as_slice() {
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Add", name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            _ => None,
        }
    }
}

fn piglatinize(s: &mut String) {
    let initial_letter = s.remove(0);
    if !is_vowel(initial_letter) {
        s.push(initial_letter);
        s.push_str("-ay");
    } else {
        s.push_str("-hay");
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
