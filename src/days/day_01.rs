use crate::util::get_counter;
use crate::util::it_to_tuple;
use crate::util::parse_grid_cols;

pub fn part_1(input: &str) -> String {
    let grid = parse_grid_cols::<i32>(input);
    let (mut list_a, mut list_b) = it_to_tuple!(grid, 2);
    list_a.sort();
    list_b.sort();
    let difference_sum = list_a
        .into_iter()
        .zip(list_b.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());
    difference_sum.to_string()
}

pub fn part_2(input: &str) -> String {
    let grid = parse_grid_cols::<i32>(input);
    let (list_a, list_b) = it_to_tuple!(grid, 2);

    let counts = get_counter(&list_b);
    let similarity_score: i32 = list_a
        .into_iter()
        .map(|a| a * counts.get(&a).unwrap_or(&0))
        .sum();
    similarity_score.to_string()
}
