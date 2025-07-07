
// This program finds the smallest positive number in a vector of integers.
fn main() {
    let numbers = vec![-10, 0, 5, 3, 12, -2, 8];

    let smallest_positive = numbers
        .iter()
        .filter(|&&x| x > 0)   // filter only positive numbers
        .min();

    match smallest_positive {
        Some(min) => println!("The smallest positive number is: {}", min),
        None => println!("No positive numbers found."),
    }
}
