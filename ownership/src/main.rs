use std::io;

fn main() {
    println!("hello, world!");
    f();
}

fn f() {
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    println!("{x}");
}