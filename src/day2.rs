use std::collections::BTreeMap;
use std::env;
use std::fs;

fn check_for2sor3s(input: String) -> (bool, bool) {
    let mut returnVal = (false, false);
    let mut sorted = input.chars().collect::<Vec<char>>();
    sorted.sort();
    // let mut last;
    let count = sorted
        .iter()
        .fold(BTreeMap::new(), |mut acc: BTreeMap<char, i32>, x: &char| {
            match acc.get(x) {
                Some(val) => acc.insert(*x, val + 1),
                None => acc.insert(*x, 1),
            };
            acc
        });
    for val in &count {
        // println!("{}: {}", val.0, val.1);
    }
    let has2s = count
        .values()
        .cloned()
        .collect::<Vec<i32>>()
        .contains(&2i32);
    let has3s = count
        .values()
        .cloned()
        .collect::<Vec<i32>>()
        .contains(&3i32);
    (has2s, has3s)
}

fn compare(this: &String, other: &String) -> Option<String> {
    let diff =
        this.chars()
            .zip(other.chars())
            .fold((String::new(), String::new()), |mut acc, x| {
                if x.0 == x.1 {
                    acc.0.push(x.0);
                    acc
                } else {
                    acc.1.push(x.0);
                    acc
                }
            });
    if diff.1.chars().count() == 1 {
        Some(diff.0)
    } else {
        None
    }
}

fn check_entry(entry: &String, input: &Vec<String>) -> Option<String> {
    let res = input
        .iter()
        .map(|x| compare(x, entry))
        .filter(|x| x.is_some())
        .collect::<Vec<Option<String>>>();
    if !res.is_empty()
    {
        res[0].clone()
    }
    else
    {
        None
    }
}

fn check_for_off_by_one(input: &Vec<String>) -> Option<String> {
    let res= input
        .iter()
        .map(|x| check_entry(x, input))
        .filter(|x| x.is_some())
        .collect::<Vec<Option<String>>>();
        if !res.is_empty()
    {
        res[0].clone()
    }
    else
    {
        None
    }
}

pub fn run() {
    let input = fs::read_to_string("./InputFiles/Day2.txt")
        .expect("Failed to read input")
        .split_whitespace()
        .map(|s| s.parse::<String>().unwrap())
        .collect::<Vec<String>>();

    let answer1 = input.iter().fold((0i32, 0i32), |mut acc, x| {
        let check_res = check_for2sor3s(x.to_string());
        if check_res.0 {
            acc.0 += 1
        };
        if check_res.1 {
            acc.1 += 1
        };
        acc
    });

    let answer2 = input.iter().fold((0i32, 0i32), |mut acc, x| {
        let check_res = check_for2sor3s(x.to_string());
        if check_res.0 {
            acc.0 += 1
        };
        if check_res.1 {
            acc.1 += 1
        };
        acc
    });
    println!(
        "Part 1: \n 2s: {} 3s: {}  Answer: {}",
        answer1.0,
        answer1.1,
        answer1.0 * answer1.1
    );
    let part2 = check_for_off_by_one(&input);
    if part2.is_some() {
        println!("Part2: {}", part2.unwrap());
    }
    return;
}
