use chapter_8::averages;

fn main() {
    let arr = vec![1, 2, 3, 3, 200, 39, 30, 2, 3, 10, 90, 1, 7, 8, 3, 9, 10];
    let median = averages::get_median(&arr).unwrap_or(0);
    let mode = averages::get_mode(&arr).unwrap_or(0);
    println!("Original array: {:?}", arr);
    println!("Median value: {}", median);
    println!("Mode value: {}", mode);
}
