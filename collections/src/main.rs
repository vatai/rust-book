// Given a list of integers, use a vector and return the mean (the
// average value), median (when sorted, the value in the middle
// position), and mode (the value that occurs most often; a hash map
// will be helpful here) of the list.

// Convert strings to pig latin. The first consonant of each word is
// moved to the end of the word and “ay” is added, so “first” becomes
// “irst-fay.” Words that start with a vowel have “hay” added to the
// end instead (“apple” becomes “apple-hay”). Keep in mind the details
// about UTF-8 encoding!

// Using a hash map and vectors, create a text interface to allow a
// user to add employee names to a department in a company. For
// example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
// let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::vec;

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for e in v {
        sum += e;
    }
    let len = v.len() as f64;
    f64::from(sum) / len
}

fn median(v: &Vec<i32>) -> i32 {
    let m = v.len() / 2;
    let mut sorted = v.clone();
    sorted.sort();
    sorted[m]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for e in v {
        let count = map.entry(e).or_insert(0);
        *count += 1;
    }
    *map.values().max().unwrap()
}

fn main() {
    let v = vec![1, 1, 3, 4, 4, 4, 4];
    println!("Mean: {}\nMedain: {}\nMode: {}\n",
             mean(&v),
             median(&v),
             mode(&v));
}
