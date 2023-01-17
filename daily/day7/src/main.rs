fn is_palindromic(n: &i32) -> bool {
    let num_as_str = n.to_string();
    let reveresed = num_as_str.chars().rev().collect::<String>();
    num_as_str == reveresed
}

fn main() {
    let output = is_palindromic(&-101);
    println!("{output}");
}
