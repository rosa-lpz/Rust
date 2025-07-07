# Smallest number from a list (vector)



## Solution 1

```rust
fn main() {
    let numbers = vec![42, 17, -93, 5, 68];

    // Use .iter().min() to find the smallest number
    if let Some(min) = numbers.iter().min() {
        println!("The smallest number is: {}", min);
    } else {
        println!("The list is empty.");
    }
}
```

**Output**

```cmd
The smallest number is: -93
```



### Explanation:

- `.iter()` gives you an iterator over references to the items in the vector.
- `.min()` returns an `Option<&T>` containing the smallest item (or `None` if the list is empty).
- We use `if let Some(min)` to safely handle the `Option`.
