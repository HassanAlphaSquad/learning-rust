fn main() {
    println!("Hello, world!");
    let a = 5;
    let b = 20;
    println!("{} + {} = {}", a, b, add(a, b))
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
