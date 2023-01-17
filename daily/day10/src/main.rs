use std::collections::HashMap;

fn sort_letters(str: &str) -> String {
    let mut chars : Vec<char> = str.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

fn group_anagrams(strs: &[&str]) -> Vec<Vec<String>> {
    let mut map = HashMap::new();
    for &str in strs {
        let sorted_word = sort_letters(&str);
        map.entry(sorted_word).or_insert(vec![]).push(str.to_string());
    }
    map.into_iter().map(|(_, value)| value).collect()
}

fn main() {
    let strs = ["a"];
    let output = group_anagrams(&strs);
    println!("{:?}", output);
}
