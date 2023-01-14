fn print(s: &String) {
    println!("{s}");
}

fn main() {
    let mut s = String::from("hello");
    print(&s);
    s.push_str(" world");
    print(&s);
}