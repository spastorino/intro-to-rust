fn main() {
    let name = String::from("Santiago");
    add_os(name);

    // Exercise 1: Think about why this does not compile
    println!("{}", name);

    let number = 1;
    add_one(number);

    // Exercise 2: Think about why does this work
    println!("{}", number);

    // Exercise 3: Think about why the printed numbers are different
}

fn add_os(mut name: String) {
    name.push_str("oooooooooooooooooo");
}

fn add_one(mut number: i32) {
    number += 1;
    println!("{}", number);
}
