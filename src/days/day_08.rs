use std::collections::HashMap;
use std::collections::HashSet;

use crate::util::get_in_range_fn;
use crate::util::indexes;
use crate::util::parse_char_grid;

fn get_antennas(grid: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();
    for (x, y) in indexes(grid.len(), grid[0].len()) {
        if grid[x][y] != '.' {
            let l = antennas.entry(grid[x][y]).or_insert(vec![]);
            l.push((x, y))
        }
    }
    antennas
}

pub fn part_1(input: &str) -> String {
    let grid = parse_char_grid(input);
    let antennas = get_antennas(&grid);
    let in_range = get_in_range_fn(grid.len(), grid[0].len());

    let mut poles = HashSet::new();
    for l in antennas.values() {
        for (a_x, a_y) in l {
            for (b_x, b_y) in l {
                if (a_x, a_y) == (b_x, b_y) {
                    continue;
                }
                let pole = ((b_x + b_x).checked_sub(*a_x), (b_y + b_y).checked_sub(*a_y));
                if let (Some(x), Some(y)) = pole {
                    if in_range(x, y) {
                        poles.insert((x, y));
                    }
                }
            }
        }
    }
    poles.len().to_string()
}

pub fn part_2(input: &str) -> String {
    let grid = parse_char_grid(input);
    let antennas = get_antennas(&grid);
    let in_range = get_in_range_fn(grid.len(), grid[0].len());

    let mut poles = HashSet::new();
    for l in antennas.values() {
        for (a_x, a_y) in l {
            for (b_x, b_y) in l {
                if (a_x, a_y) == (b_x, b_y) {
                    continue;
                }
                let mut next_pole = (Some(*a_x), Some(*a_y));
                while let (Some(x), Some(y)) = next_pole {
                    if in_range(x, y) {
                        poles.insert((x, y));
                        next_pole = ((x + b_x).checked_sub(*a_x), (y + b_y).checked_sub(*a_y));
                    } else {
                        break;
                    }
                }
            }
        }
    }
    poles.len().to_string()
}
