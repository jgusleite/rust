use std::env::*;

fn main() {
    let args:= std::env::args().skip(1);
    for arg in args {
        print!("{arg"});
    }
}
