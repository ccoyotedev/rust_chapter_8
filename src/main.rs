use chapter_8::averages;

fn main() {
    let arr = vec![1, 2, 3, 3, 200, 39, 30, 2, 3, 10, 90, 1, 7, 8];
    let median = averages::get_median(&arr).unwrap_or(0);
    println!("Original array: {:?}", arr);
    println!("Median value: {}", median);
}
