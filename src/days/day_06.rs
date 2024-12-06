use crate::util::get_in_range_fn;
use crate::util::indexes;
use crate::util::parse_char_grid;
use std::collections::HashSet;

pub fn get_visited(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let in_range = get_in_range_fn(n, m);

    let (mut x, mut y) = indexes(n, m).find(|&(i, j)| grid[i][j] == '^').unwrap();
    let mut dir = 3;
    let mut seen = HashSet::new();
    loop {
        seen.insert((x, y));
        let (res_x, res_y) = (x.checked_add_signed(dx[dir]), y.checked_add_signed(dy[dir]));
        if let (Some(new_x), Some(new_y)) = (res_x, res_y) {
            if !(in_range(new_x, new_y)) {
                break;
            } else if grid[new_x][new_y] == '#' {
                dir = (dir + 1) % 4;
            } else {
                (x, y) = (new_x, new_y);
            }
        } else {
            break;
        }
    }
    seen
}

pub fn has_loop(grid: &[Vec<char>]) -> bool {
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let in_range = get_in_range_fn(n, m);

    let (mut x, mut y) = indexes(n, m).find(|&(i, j)| grid[i][j] == '^').unwrap();
    let mut dir = 3;
    let mut seen = HashSet::new();
    loop {
        let (res_x, res_y) = (x.checked_add_signed(dx[dir]), y.checked_add_signed(dy[dir]));
        if let (Some(new_x), Some(new_y)) = (res_x, res_y) {
            if !(in_range(new_x, new_y)) {
                break;
            } else if grid[new_x][new_y] == '#' {
                if !seen.insert((x, y, dir)) {
                    return true;
                }
                dir = (dir + 1) % 4;
            } else {
                (x, y) = (new_x, new_y);
            }
        } else {
            break;
        }
    }
    false
}

pub fn part_1(input: &str) -> String {
    let grid = parse_char_grid(input);
    get_visited(&grid).len().to_string()
}

pub fn part_2(input: &str) -> String {
    let mut grid = parse_char_grid(input);
    let seen = get_visited(&grid);
    let mut count = 0;

    for (x, y) in seen {
        if grid[x][y] == '^' {
            continue;
        }
        grid[x][y] = '#';
        count += has_loop(&grid) as i32;
        grid[x][y] = '.';
    }
    count.to_string()
}
