use crate::util::it_to_tuple;
use std::{cmp::Ordering, collections::HashSet};

fn parse(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules, nums) = it_to_tuple!(input.split("\n\n"), 2);
    let ruleset: HashSet<(i32, i32)> = rules
        .lines()
        .map(|l| it_to_tuple!(l.split("|").map(|i| i.parse().unwrap()), 2))
        .collect();
    let parsed_nums = nums
        .lines()
        .map(|l| l.split(",").map(|i| i.parse().unwrap()).collect::<Vec<_>>())
        .collect();
    (ruleset, parsed_nums)
}

pub fn part_1(input: &str) -> String {
    let (ruleset, nums) = parse(input);
    nums.iter()
        .filter(|v| {
            for (i, &num1) in v.iter().enumerate() {
                for &num2 in &v[i + 1..] {
                    if ruleset.contains(&(num2, num1)) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|v| v[v.len() / 2])
        .sum::<i32>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let (ruleset, nums) = parse(input);
    let wrong_lines: Vec<Vec<i32>> = nums
        .into_iter()
        .filter(|v| {
            for (i, &num1) in v.iter().enumerate() {
                for &num2 in &v[i + 1..] {
                    if ruleset.contains(&(num2, num1)) {
                        return true;
                    }
                }
            }
            false
        })
        .collect();

    wrong_lines
        .into_iter()
        .map(|mut l| {
            l.sort_by(|&a, &b| {
                if a == b {
                    Ordering::Equal
                } else if ruleset.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            l[l.len() / 2]
        })
        .sum::<i32>()
        .to_string()
}
