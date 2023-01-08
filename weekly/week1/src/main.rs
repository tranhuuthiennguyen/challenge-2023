fn reverse_string(str: &String) -> String {
    let mut new_string = String::new();
    let mut i = (*str).len();

    while i != 0 {
        i -= 1;
        let chr = (*str).as_bytes()[i];
        new_string.push(chr as char);
    }

    new_string
}

fn is_palindrome(str: &String) -> bool {
    let reverse = reverse_string(&str);
    reverse == *str
}



fn main() {
    let str = String::from("bab");
    println!("{}", is_palindrome(&str));
}
