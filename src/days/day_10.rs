use crate::util::indexes;
use std::collections::HashSet;

pub fn part_1(input: &str) -> String {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let in_range = |x: usize, y: usize| (0..grid.len()).contains(&x) && (0..grid[0].len()).contains(&y);

    let successors = |x: usize, y: usize| {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .filter_map(
                move |(dx, dy)| match (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                    (Some(x), Some(y)) if in_range(x, y) => Some((x, y)),
                    _ => None,
                },
            )
    };

    let mut reachable: Vec<Vec<Option<HashSet<(usize, usize)>>>> =
        vec![vec![None; grid[0].len()]; grid.len()];
    let mut stack: Vec<_> = indexes(grid.len(), grid[0].len())
        .filter(|&(x, y)| grid[x][y] == 0)
        .collect();

    while stack.len() > 0 {
        let &(x, y) = stack.last().unwrap();

        if reachable[x][y].is_some() {
            for (nx, ny) in successors(x, y) {
                reachable[x][y].unwrap().extend(reachable[nx][ny].as_ref().unwrap().iter());
            }
            stack.pop();
            continue;
        }
        if grid[x][y] == 9 {
            reachable[x][y] = Some(HashSet::from([(x, y)]));
            stack.pop();
            continue;
        }

        reachable[x][y] = Some(HashSet::new());
        for (nx, ny) in successors(x, y) {
            if reachable[nx][ny].is_none() {
                stack.push((nx, ny));
            }
        }
    }

    indexes(grid.len(), grid[0].len())
        .filter(|&(x, y)| grid[x][y] == 0)
        .map(|(x, y)| reachable[x][y].unwrap().len())
        .sum::<usize>()
        .to_string()
}

#[allow(unused_variables)]
pub fn part_2(input: &str) -> String {
    String::new()
}
