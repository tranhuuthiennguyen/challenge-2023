use std::vec::Vec;

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {   
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn is_valid_paren(string: &str) -> bool {
    let mut stack : Stack<char> = Stack::new();

    let char_vec : Vec<char> = string.chars().collect();

    for ch in char_vec {
        if ch == '(' || ch == '[' || ch == '{' {
            stack.push(ch);
        } else {
            let top = stack.peek().unwrap();
            if (*top == ')' && ch != '(') ||
                (*top == '[' && ch != ']' ||
                (*top == '{' && ch != '}')) {
                    continue;
                }
            else {
                stack.pop();
            }
        }
    }

    stack.is_empty()
}

fn main() {
    let output = is_valid_paren(&"([()})");
    println!("{output}");
}
