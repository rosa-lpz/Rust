# Rust
Rust basics





## Basic Concepts

**Rust** is a systems programming language focused on:

- **Safety** (no nulls, no data races)
- **Speed**
- **Concurrency**

It’s a great alternative to C/C++ for safe, fast code.

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



- 

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







Sure! Here's a concise overview of **Rust basics**, perfect if you're getting started with the language.

------



------

## **Data Types**

Rust is statically typed:

```rust
let a: i32 = 10;      // 32-bit integer
let b: f64 = 3.14;    // 64-bit float
let c: bool = true;   // Boolean
let d: char = '🦀';    // Unicode char
```

------

## Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return
}
```

- Return type after `->`.
- Last expression is implicitly returned.

------

## **Control Flow**

```rust
let n = 5;

if n > 0 {
    println!("Positive");
} else {
    println!("Zero or negative");
}

for i in 0..5 {
    println!("{}", i);
}

let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}
```

------

## **Structs**

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{} is {} years old", p.name, p.age);
}
```

------

## **Enums & Pattern Matching**

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_it(dir: Direction) {
    match dir {
        Direction::North => println!("Up!"),
        Direction::South => println!("Down!"),
        _ => println!("Sideways!"),
    }
}
```

------

## **Option & Result**

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}
```

Rust uses `Option<T>` instead of nulls.

```rust
use std::fs::File;

fn main() {
    let f = File::open("data.txt");

    match f {
        Ok(file) => println!("Opened file"),
        Err(e) => println!("Error: {}", e),
    }
}
```

`Result<T, E>` handles success or error.

------

## **Ownership, Borrowing, and Lifetimes (Core Concepts)**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // ownership moved, s1 is invalid now
    // println!("{}", s1); // Error!

    let s3 = s2.clone(); // s3 is a deep copy
}
```

- Variables own their data.
- Data is moved (not copied) unless explicitly cloned.
- Use references to **borrow**:

```rust
fn print_length(s: &String) {
    println!("Length is {}", s.len());
}
```

------

