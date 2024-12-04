pub fn part_1(input: &str) -> String {
    let grid: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();

    let targets = [b"XMAS", b"SAMX"];
    let dirs = [(0, 1), (1, 0), (1, 1), (1, -1)];

    let check = |y: usize, x: usize, dx: i32, dy: i32, target: &[u8]| {
        for i in 0..4 {
            let yy = y as i32 + i*dy;
            let xx = x as i32 + i*dx;

            if !(0 <= yy && yy < grid.len() as i32 && 0 <= xx && xx < grid[0].len() as i32) {
                return false;
            }
            if grid[yy as usize][xx as usize] != target[i as usize] {
                return false;
            }
        }
        true
    };

    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for &target in &targets {
                for (dy, dx) in dirs {
                    if check(y, x, dx, dy, target) {
                        count += 1
                    }
                }
            }
        }
    }
    count.to_string()
}

pub fn part_2(input: &str) -> String {
    let grid: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();

    let targets = [b"MAS", b"SAM"];

    let check = |y: usize, x: usize, t1: &[u8], t2: &[u8]| {
        for i in 0..3 {
            if grid[y+i][x+i] != t1[i] || grid[y+i][x+2-i] != t2[i] {
                return false;
            }
        }
        true
    };

    let mut count = 0;

    for y in 0..grid.len()-2 {
        for x in 0..grid[0].len()-2 {
            for &t1 in &targets {
                for &t2 in &targets {
                    if check(y, x, t1, t2) {
                        count += 1;
                    }
                }
            }
        }
    }
    count.to_string()
}
