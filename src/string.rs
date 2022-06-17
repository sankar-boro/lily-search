use std::iter::FromIterator;

const ignored_words: [&str; 26] = [
    "a", "be", "by", "the", "of", 
    "of", "at", "its", "do", "is",
    "he", "she", "it", "as", "to", 
    "they", "from", "and", "for", "on",
    "them", "should", "has",  "have", "shall",
    "in",
];


pub fn split_string<T: FromIterator<String>>(data: &str) -> T {
    let d: T = data.split(' ').map(|d| -> String {d.to_owned()}).collect();
    return d;
}

pub fn ignore_strings(data: &Vec<String>) -> Vec<String> {
    let ig_words: [String; ignored_words.len()] = ignored_words.map(|d| {d.to_owned()}); 
    let mut new_vec: Vec<String> = Vec::new();
    for w in data {
        let a = &w.to_lowercase();
        if !ig_words.contains(a) {
            new_vec.push(clean_string(a));
        }
    }
    new_vec
}

pub fn clean_string(data: &str) -> String {
    let d: &[u8] = data.as_bytes();
    let a = d.iter().map(|x| { if x.clone() > 96 && x.clone() < 123 || x.clone() == 32 { x.clone() } else { 32 } }).collect::<Vec<_>>();
    let b = String::from_utf8(a).unwrap();
    b
}