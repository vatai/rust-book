// Given a list of integers, use a vector and return the mean (the
// average value), median (when sorted, the value in the middle
// position), and mode (the value that occurs most often; a hash map
// will be helpful here) of the list.

use std::collections::HashMap;
use std::vec;

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for e in v {
        sum += e;
    }
    f64::from(sum)
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
    let v = vec![1, 4, 3, 4, 4, 4, 4];
    println!("Mean: {}\nMedain: {}\nMode: {}\n",
             mean(&v),
             median(&v),
             mode(&v));
}
