#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<i32, &'static str> = {
        let map = HashMap::from([
            (1, "I"),
            (5, "V"),
            (10, "X"),
            (50, "L"),
            (100, "C"),
            (500, "D"),
            (1000, "M"),
        ]);
        map
    };
}

fn integer_to_roman(n: &mut i32) -> String {
    let mut res = String::new();
    let mut div = 1;
    while div < (*n)/10 {
        div*=10;
    }
    while *n > 0 {
        let last_number = *n/div;

        if last_number <= 3 {
            let new_code = HASHMAP.get(&div).unwrap().repeat(last_number as usize);
            res.push_str(&new_code);
        } else if last_number == 4 {
            res.push_str(
                HASHMAP.get(&div).unwrap()
            );
            res.push_str(
                HASHMAP.get(&(div * 5)).unwrap()
            );
        } else if 5 <= last_number && last_number <= 8 {
            res.push_str(HASHMAP.get(&(div*5)).unwrap());
            res.push_str(&HASHMAP.get(&div).unwrap().repeat((last_number-5) as usize));
        } else if last_number == 9 {
            res.push_str(HASHMAP.get(&div).unwrap());
            res.push_str(
                HASHMAP.get(&(div*10)).unwrap()
            );
        }
        *n%=div;
        div/=10;
    }
    res
}

fn main() {
    let mut num : i32 = 1997;
    let output = integer_to_roman(&mut num);
    println!("{output}");
}
