use std::{collections::HashMap, iter::zip};

pub fn p1(input: &str) -> i32 {
    let (mut c1, mut c2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|c| {
                    c.parse::<i32>()
                        .map_err(|e| format!("Failed to parse integer: {}", e.to_string()))
                })
                .collect::<Result<Vec<i32>, String>>()
        })
        .map(|l| (l[0], l[1]))
        .unzip();

    c1.sort_unstable();
    c2.sort_unstable();

    zip(c1, c2).map(|(c1, c2)| (c1 - c2).abs()).sum()
}

pub fn p2(input: &str) -> i32 {
    let (c1, c2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|c| {
                    c.parse::<i32>()
                        .map_err(|e| format!("Failed to parse integer: {}", e.to_string()))
                })
                .collect::<Result<Vec<i32>, String>>()
        })
        .map(|l| (l[0], l[1]))
        .unzip();

    let mut appearances = HashMap::new();

    for i in c2 {
        if let Some(sum) = appearances.get(&i) {
            appearances.insert(i, sum + 1);
        } else {
            appearances.insert(i, 1);
        }
    }

    c1.into_iter()
        .map(|i| i * appearances.get(&i).unwrap_or(&0))
        .sum()
}

pub fn p2_2(input: &str) -> i32 {
    let (c1, c2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|c| {
                    c.parse::<i32>()
                        .map_err(|e| format!("Failed to parse integer: {}", e.to_string()))
                })
                .collect::<Result<Vec<i32>, String>>()
        })
        .map(|l| (l[0], l[1]))
        .unzip();

    let appearances = c2.into_iter().fold(HashMap::new(), |mut acc, i| {
        if let Some(sum) = acc.get(&i) {
            acc.insert(i, sum + 1);
            acc
        } else {
            acc.insert(i, 1);
            acc
        }
    });

    c1.into_iter()
        .map(|i| i * appearances.get(&i).unwrap_or(&0))
        .sum()
}
