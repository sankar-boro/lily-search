use std::iter::FromIterator;

pub fn split_string<T: FromIterator<String>>(data: &str) -> T {
    let d: T = data.split(' ').map(|d| -> String {d.to_owned()}).collect();
    return d;
}