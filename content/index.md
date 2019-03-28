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

---

# Schedule

---

Explain different parts of the schedule

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

# Install Rust

- Install nightly Rust using [rustup](https://rustup.sh) or any other way.
- Run `rustc -V` to see if you’re golden.

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
    } else if == 5 {
        // ...
    } else {
        /* ... */
    }
}
```

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

# Struct

```rust
struct Empty;

struct User {
    user: String,
    age: u32,
}

struct Pair(i32, i32);
```

---

# Type alias

```rust
type Explanation = String;
```

---

# Enum

```rust
enum Choice {
    Yes,
    No,
    Maybe(Explanation),
}
```

---

# Exercise 1

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
