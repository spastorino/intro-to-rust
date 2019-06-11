// Exercise 1: Is the compiler right to complain about your code?
// Exercise 2: Make the code compile

fn main() {
    let vector = vec![1, 2, 3, 4];
    println!("Half of it is: {:?}", half(&vector));
}

fn half(vector: &Vec<i32>) -> &Vec<i32> {
    let middle = vector.len()/2 + 1;
    let mut half = Vec::new();
    for i in 0..middle {
        half.push(vector[i]);
    }
    &half
}
