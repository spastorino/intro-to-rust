class: center
name: title
count: false

<img src="content/images/rust-logo-blk.svg" alt="Rust logo" width="250rem" height="auto">

# Intro to Rust Workshop

.grey[Santiago Pastorino]

---

# About me

- Computer Science at Fing
- WyeWorks co-founder
- Ruby on Rails core team alumni
- Following Rust since 2014
- Started working with Rust in 2017
- Member of Rust compiler NLL WG
- Rust Latam conference organizer
- Rust Montevideo Meetup organizer

<div style="height: 2rem"></div>

Contact:

- Twitter/Github: spastorino
- Email: spastorino@gmail.com

---

# About the workshop

- Covering some syntax
- Basic concepts

---

# What is Rust?

- "New" safe systems programming language
- Developed by Mozilla research, v1.0 released in 2015
- Multiparadigm
- Imperative, Structured, Functional, Concurrent, Generic, Compiled
- Static strong Typing
- Inference
- Emphasizing control, safety, and speed
- Free and open-source software, MIT License or Apache License 2.0
- Most loved programming language (2016, 2017 & 2018)

---

# Why Rust?

- Performance
- Reliability
- Productivity

---

# Performance

Rust is **blazingly fast** and **memory-efficient**: with **no runtime or garbage collector**, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

---

# Reliability

Rust’s **rich type system and ownership** model guarantee **memory-safety and thread-safety** — and enable you to eliminate many classes of bugs at compile-time.

---

# Productivity

Rust has **great documentation**, a **friendly compiler** with useful error messages, and **top-notch tooling** — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

---

# Main use cases

- Systems programming in general
  - Operating Systems
  - Browsers
  - Servers
- Address hotspots of your app (Ruby, JavaScript, Python, etc)
- Web Services, WebAssembly
- Networking, Blockchain
- Embedded, Microcontrollers, IoT
- Games
- Machine learning
- Cli

---

# Who uses Rust?

- Mozilla (Stylo, WebRender)
- Google (Fuschia operating system)
- Facebook (Mercurial rewrite)
- Amazon (Firecracker)
- Microsoft (Azure IoT)
- Dropbox (storage system)
- Npm (npm core services)
- Red Hat (storage system)

---

# Installation

- Install Rust using [https://rustup.rs](https://rustup.rs) or any other way.
- Run `rustc -V` to see if you’re golden.
- Ensure you have git installed
- git clone https://github.com/spastorino/intro-to-rust.git

---

# Create a project with cargo

```shell
$ cargo new myapp
$ cd myapp
$ cargo build
$ cargo run
$ cargo doc
# => Hello, world!
```

---

# Functions

```rust
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = 10;
    let y = 15;

    let z = add_numbers(x, y);
    println!("{} + {} = {}", x, y, z);
}
```

---

# If

```rust
fn main() {
    let value = 10;

    if value % 2 == 0 {
        // ...
    } else if value == 5 {
        // ...
    } else {
        /* ... */
    }
}
```

---

# Exercise 1

- Go to this `intro-to-rust/exercises/1_if` project
- Try running `cargo test`
- Make tests pass :)

---

# Loop

```rust
fn main() {
    let mut value = 0;

    // Loop with break
    loop {
        if value >= 10 {
        break;
    }

    value += 1;
    }
}
```

---

# While

```rust
fn main() {
    let mut value = 0;

    // Break on conditional
    while value < 10 {
        value += 1;
    }
}
```

---

# For

```rust
fn main() {
    let mut value = 0;

    // Loop over iterator
    for i in 0..10 {
        value += i;
    }
}
```

---

# Exercise 2

- Go to this `intro-to-rust/exercises/2_loops` project
- Try running `cargo test`
- Make tests pass :)

---

# Struct

```rust
struct Location {
    longitude: f32,
    latitude: f32,
}

impl Location {
    fn new(longitude: f32, latitude: f32) -> Location {
        Location { longitude, latitude }
    }

    fn longitude(&self) -> f32 {
        self.longitude
    }

    fn latitude(&self) -> f32 {
        self.latitude
    }

    fn move_location(&mut self, other: Location) {
        self.longitude += other.longitude();
        self.latitude += other.latitude();
    }
}
```

---

# Exercise 3

- Go to this `intro-to-rust/exercises/3_structs` project
- Open `src/main.rs` you will find guidance there

---

# Enum

```rust
struct Location {
    longitude: f32,
    latitude: f32,
}

struct Map {
    pin: Option<Location>,
    weather: Weather,
}

enum Weather {
    Sunny,
    Windy,
    Rainy,
    Snowy,
}
```

---

# Exercise 4

- Go to this `intro-to-rust/exercises/4_enums` project
- Open `src/main.rs` you will find guidance there

---

# Memory safety without garbage collection

- No segmentation faults
- No double free
- No dangling pointers
- No iterator invalidation
- No buffer overflows
- No undefined behavior
- No null pointers
- No data races
- Guaranteed by Rust's ownership system at compile time

???

- Every malloc needs one free
- Array capacity is not checked
- Vulnerabilities caused by memory unsafety are still common

---

# "Manual" memory management in Rust:

- Values **owned** by creator.
- Values **moved** via assignment.
- When final owner returns, **value is freed**.

All this feels invisible and prevents _double free_ errors and _memory leaks_.

???

- Move semantics / RAII
- Rust enforces the RAII discipline
- Variables can own resources

---

# Ownership

```rust
fn main() {
    let apple_1 = Apple::new();
    eat(apple_1); // Give ownership of the apple_1.
    eat(apple_1); // Error: apple_1 has been moved.
}

/// eat function takes ownership of the apple
fn eat(apple_2: Apple) {
}
```

---

# Ownership

<img src="content/images/rust-meetup-children-ownership-0r.jpg" alt="Ownership 0" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
    let apple = Apple::new();
    let mut bag = Vec::new();
    bag.push(apple); // Give ownership
    bag.push(Apple::new());
    deliver(bag); // Give ownership of bag and it's contents
}

/// deliver function takes ownership
/// of the vector
fn deliver(bag: Vec<Apple>) {
    // ...
}
```

---

# Ownership

<img src="content/images/rust-meetup-children-ownership-1r.png" alt="Ownership 1" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
*   let apple = Apple::new();
    let mut bag = Vec::new();
    bag.push(apple); // Give ownership
    bag.push(Apple::new());
    deliver(bag); // Give ownership of bag and it's contents
}

/// deliver function takes ownership
/// of the vector
fn deliver(bag: Vec<Apple>) {
    // ...
}
```

---

# Ownership

<img src="content/images/rust-meetup-children-ownership-2r.png" alt="Ownership 2" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
    let apple = Apple::new();
*   let mut bag = Vec::new();
    bag.push(apple); // Give ownership
    bag.push(Apple::new());
    deliver(bag); // Give ownership of bag and it's contents
}

/// deliver function takes ownership
/// of the vector
fn deliver(bag: Vec<Apple>) {
    // ...
}
```

---

# Ownership

<img src="content/images/rust-meetup-children-ownership-3r.png" alt="Ownership 3" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
    let apple = Apple::new();
    let mut bag = Vec::new();
*   bag.push(apple); // Give ownership
    bag.push(Apple::new());
    deliver(bag); // Give ownership of bag and it's contents
}

/// deliver function takes ownership
/// of the vector
fn deliver(bag: Vec<Apple>) {
    // ...
}
```

---

# Ownership

<img src="content/images/rust-meetup-children-ownership-4r.png" alt="Ownership 4" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
    let apple = Apple::new();
    let mut bag = Vec::new();
    bag.push(apple); // Give ownership
*   bag.push(Apple::new());
    deliver(bag); // Give ownership of bag and it's contents
}

/// deliver function takes ownership
/// of the vector
fn deliver(bag: Vec<Apple>) {
    // ...
}
```

---

# Ownership

<img src="content/images/rust-meetup-children-ownership-6r.png" alt="Ownership 6" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
    let apple = Apple::new();
    let mut bag = Vec::new();
    bag.push(apple); // Give ownership
    bag.push(Apple::new());
*   deliver(bag); // Give ownership of bag and it's contents
}

/// deliver function takes ownership
/// of the vector
fn deliver(bag: Vec<Apple>) {
    // ...
}
```

---

# Exercise 5

- Go to this `intro-to-rust/exercises/5_ownership/src` project
- Open `src/main.rs` you will find guidance there

---

# What if I want to use bag again?

```rust
fn main() {
    let apple = Apple::new();
    let mut bag = Vec::new();
    bag.push(apple);
    bag.push(Apple::new());
*   let (weight, bag) = weight(bag); // Return the bag back
    println!("Bag {}, weights {}", bag, weight);
}

/// weight function takes an owned bag
/// and return its weight and the bag back
fn weight(bag: Vec<Apple>) -> (u32, Vec<Apple>) {
    // ...
}
```

---

# Borrowing

<img src="content/images/rust-meetup-children-borrowing-0r.png" alt="Borrowing" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
    let apple = Apple::new();
    let mut bag = Vec::new();
    bag.push(apple);
    bag.push(Apple::new());
*   let weight = weight(&bag); // Borrow the bag
    println!("Bag {}, weights {}", bag, weight);
}

/// weight function takes a shared
/// reference to the vector
fn weight(bag: &Vec<Apple>) -> u32 {
    // ...
}
```

---

# Mutable borrowing

<img src="content/images/rust-meetup-children-borrowing-0r.png" alt="Borrowing" width="300rem" height="auto" style="position: absolute; right: 3rem; margin-top: 0rem">

```rust
fn main() {
    let apple = Apple::new();
    let mut bag = Vec::new();
    bag.push(apple);
    bag.push(Apple::new());
*   deliver(&mut bag);        // Borrow the bag for mutation
    println!("Bag is now {}", bag);
}

/// deliver function takes a mutable shared
/// reference to the vector
fn deliver(bag: &mut Vec<Apple>) {
    // mutate the bag
}
```

---

# Dangers of mutation

```rust
let mut buffer = format!("Hello");
let slice = &buffer[1..];
buffer.push_str(" World");
println!("{:?}", slice);
```

---

# Dangers of mutation

```rust
*let mut buffer = format!("Hello");
let slice = &buffer[1..];
buffer.push_str(" World");
println!("{:?}", slice);
```

<img src="content/images/rust-meetup-mutation-1r.png" alt="Mutation 1">

---

# Dangers of mutation

```rust
let mut buffer = format!("Hello");
*let slice = &buffer[1..];
buffer.push_str(" World");
println!("{:?}", slice);
```

<img src="content/images/rust-meetup-mutation-2r.png" alt="Mutation 2">

---

# Dangers of mutation

```rust
let mut buffer = format!("Hello");
let slice = &buffer[1..];
*buffer.push_str(" World");
println!("{:?}", slice);
```

<img src="content/images/rust-meetup-mutation-3r.png" alt="Mutation 3">

---

# Dangers of mutation

```rust
let mut buffer = format!("Hello");
let slice = &buffer[1..];
*buffer.push_str(" World");
println!("{:?}", slice);
```

<img src="content/images/rust-meetup-mutation-4r.png" alt="Mutation 4">

---

# Dangers of mutation

```rust
let mut buffer = format!("Hello");
let slice = &buffer[1..];
*buffer.push_str(" World");
println!("{:?}", slice);
```

<img src="content/images/rust-meetup-mutation-5r.png" alt="Mutation 5">

---

# Dangers of mutation

```rust
let mut buffer = format!("Hello");
let slice = &buffer[1..];
*buffer.push_str(" World");
println!("{:?}", slice);
```

<img src="content/images/rust-meetup-mutation-6r.png" alt="Mutation 6">

???

- No aliasing + mutation at the same time

---

# Lifetime of a borrow

```rust
let mut buffer = format!("Hello");
*let slice = &buffer[1..];
*buffer.push_str(" World");
*println!("{:?}", slice);
```

**Lifetime**: span of code where reference is used.

--

**Rules**:

- If there's a shared reference, no writers during the **lifetime of the shared borrow**.
- If there's a mutable reference, no other readers or writers during the **lifetime of the mutable borrow**

---

# Exercise 6

- Go to this `intro-to-rust/exercises/6_borrowing/src` project
- Open `src/main.rs` you will find guidance there

---

# Generics

```rust
enum Choice<T> {
    Yes,
    No,
    Maybe(T),
}
```

---

# Option<T>

```rust
enum Option<T> {
    Some(T),
    None,
}
```

???

null vs Option

---

# Match

```rust
fn main() {
    let value = "hello";

    match value {
        "hello" => {
            println!("Hello world!");
        }

        "bye" => {
            println!("Bye world!");
        }

        _ => {
            println!("Don't know what you meant");
        }
    }
}
```

---

# Strings (&str vs String)

- &str static and read-only
- String dynamic

```rust
fn main() {
    let a = "hi"; // &str

    a.push_str("something"); // compile error

    let b = String::from("hi"); // String

    b.push_str(" world");

}
```

???

UTF-8

---

# Exercise 7 - Build your own shell

- Write a shell which can run a single command on a separate process.

---

## <center>Build your own shell</center>

``` rust
fn main() {
   loop {
       // Read line from standard input
       // "Parse" line into executable command
       // Execute the command in a separate process
       // Show output
   }
}
```
