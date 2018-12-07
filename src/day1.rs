use std::collections::HashSet;
use std::env;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("./InputFiles/Day1.txt")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut freqs = HashSet::new();
    let mut freq = 0i32;
    // let min = input.iter().min().unwrap();
    let mut count = 0;
    loop {
        for number in &input {
            freq = freq + number;
            // if freqs.contains(&freq) {
            //     println!("Repeated {}", freq);
            //     return ();
            // }
            if freqs.replace(freq).is_some() {
                println!("Repeated {}", freq);
                return ();
            }
        }
        count = count + 1;
        println!("{} loops, cutoff: ", count);
    }
}
