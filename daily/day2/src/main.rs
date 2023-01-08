use std::vec::Vec;

fn main() {
    let mut n = 11;
    let mut vec = Vec::new();
    while n != 1 {
        if n % 2 == 0 {
            n/=2;
        } else {
            n = 3*n+1;
        }
        vec.push(n);
    }

    for i in vec {
        println!("{}", i);
    }
}
