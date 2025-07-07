fn main() {
    let numbers = vec![42, 17, -93, 5, 68];

    // Use .iter().min() to find the smallest number
    if let Some(min) = numbers.iter().min() {
        println!("The smallest number is: {}", min);
    } else {
        println!("The list is empty.");
    }
}