use std::iter::FromIterator;

const ignored_words: [&str; 25] = [
    "a", "be", "by", "the", "of", 
    "of", "at", "its", "do", "is",
    "he", "she", "it", "as", "to", 
    "them", "should", "has",  "have", "shall",
    "they", "from", "and", "for", "on",
];


pub fn split_string<T: FromIterator<String>>(data: &str) -> T {
    let d: T = data.split(' ').map(|d| -> String {d.to_owned()}).collect();
    return d;
}

pub fn ignore_strings(data: &Vec<String>) -> Vec<String> {
    let ig_words: [String; ignored_words.len()] = ignored_words.map(|d| {d.to_owned()}); 
    let mut new_vec: Vec<String> = Vec::new();
    for w in data {
        if !ig_words.contains(w) {
            new_vec.push(w.to_owned());
        }
    }
    new_vec
}