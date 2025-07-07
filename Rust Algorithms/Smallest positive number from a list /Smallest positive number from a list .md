# Smallest positive number from a list (vector)



## Solution 1

```rust
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
```

**Output**

```cmd
The smallest number is: 3
```



### Explanation:

- `filter(|&&x| x > 0)` selects only numbers greater than 0.
- `.min()` finds the smallest among them.
- The result is still an `Option<&i32>`, so we use pattern matching with `match`.
