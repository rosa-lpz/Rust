# Rust
Rust basics





## Basic Concepts



## First program

```rust
fn main() {
    println!("Hello, world!");
}
```





## Variables

### Binding and mutability 

A variable can be used only if it has been initialized.

```rust
fn main() {
    let x: i32 =5; // initialized but used
    let _y: i32; // initialized but also unused

    assert_eq!(x, 5);
    println!("Success!");
}
```

**Output**

```cmd
Success!
```





 Use `mut` to mark a variable as mutable.

```rust
// Fill the blanks in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x += 2;  //3
    
    assert_eq!(x, 3);
    println!("Success!");
}
```

**Output**

```cmd
Success!
```



### Scope

A scope is the range within the program for which the item is valid.



```rust
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}
```

