use std::io;

use chapter_8::averages;
use chapter_8::pig_latin;

fn main() {
    question_2();
}

fn question_1() {
    let arr = vec![1, 2, 3, 3, 200, 39, 30, 2, 3, 10, 90, 1, 7, 8, 3, 9, 10];
    let median = averages::get_median(&arr).unwrap_or(0);
    let mode = averages::get_mode(&arr).unwrap_or(0);
    println!("Original array: {:?}", arr);
    println!("Median value: {}", median);
    println!("Mode value: {}", mode);
}

fn question_2() {
    loop {
        println!("Input word you want to translate to pig latin");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: String = match input.trim().parse() {
            Ok(string) => string,
            Err(_) => {
                println!("Please enter valid word");
                continue;
            }
        };

        let translation = pig_latin::translate_string(input);

        println!("{}", translation);

        break;
    }
}
