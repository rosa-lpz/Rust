## Functions

Functions are prevalent in Rust code. 

* The **`main`** function, is the entry point of many programs. 
* **`fn`** keyword, which allows you to declare new functions.

**Syntaxis**

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```



**Example**

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return
}
```

- Return type after `->`.
- Last expression is implicitly returned.
