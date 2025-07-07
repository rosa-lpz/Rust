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

fn main(){
    println!("Hello, world!");
    let s: String::from("value");
}
```

## Execute program

To create a new Rust project in the terminal, follow these steps:

### 1. **Install Rust (if you haven't already)**

If you don’t have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal or run:

```bash
source $HOME/.cargo/env
```

------

### 2. **Create a New Project**

Use `cargo`, Rust’s package manager and build tool:

```bash
cargo new my_project
```

This will create a new directory called `my_project` with the following structure:

```
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

------

### 3. **Navigate to the Project Directory**

```bash
cd my_project
```

------

### 4. **Build and Run the Project**

To build:

```bash
cargo build
```

To run:

```bash
cargo run
```

This will compile and execute the default `Hello, world!` program.

------

Let me know if you want to make it a **library project** instead, or add dependencies like `serde`, etc.



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

Every value in Rust is of a certain *data type*, which tells Rust what kind of data is being specified so it knows how to work with that data

Rust is a *statically typed* language, which means that it must know the types of all variables at compile time. 



```rust
let a: i32 = 10;      // 32-bit integer
let b: f64 = 3.14;    // 64-bit float
let c: bool = true;   // Boolean
let d: char = '🦀';    // Unicode char
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





# References

* Let's Get Rusty - Ultimate VS Code setup for Rust development (2025): https://youtu.be/ZhedgZtd8gw
