use itertools::Itertools;

pub fn get_files(input: &str) -> Vec<(u64, u64, u64)> {
    let mut files = Vec::with_capacity(input.len() / 2);
    let mut is_file = true;
    let mut pos = 0;
    let mut id = 0;

    for c in input.trim().chars() {
        let len = c.to_digit(10).unwrap() as u64;
        if is_file {
            files.push((id, pos, len));
            id += 1;
        }
        pos += len;
        is_file = !is_file;
    }
    files
}

fn get_checksum(files: &[(u64, u64, u64)]) -> u64 {
    files
        .iter()
        .map(|(id, pos, len)| id * (len * pos + (len - 1) * len / 2))
        .sum::<u64>()
}

pub fn part_1(input: &str) -> String {
    let mut files = get_files(input);
    let mut compacted_files = Vec::with_capacity(files.len());
    compacted_files.push(files[0]);
    let mut i = 1;

    while i < files.len() {
        let (_, pos, len) = compacted_files.last().unwrap();
        let gap = files[i].1 - pos - len;
        if gap > 0 {
            let (back_id, back_pos, back_len) = files.pop().unwrap();
            if gap >= back_len {
                compacted_files.push((back_id, pos + len, back_len));
            } else {
                compacted_files.push((back_id, pos + len, gap));
                files.push((back_id, back_pos, back_len - gap));
            }
        } else {
            compacted_files.push(files[i]);
            i += 1;
        }
    }

    get_checksum(&compacted_files).to_string()
}

pub fn part_2(input: &str) -> String {
    // idea copied from reddit: https://old.reddit.com/r/adventofcode/comments/1ha27bo/2024_day_9_solutions/m15sexj/
    let mut files = get_files(input);
    let mut spaces: Vec<_> = files
        .iter()
        .tuple_windows()
        .map(|((_, pos1, len1), (_, pos2, _))| (pos1 + len1, pos2 - pos1 - len1))
        .collect();

    for i in (0..files.len()).rev() {
        let (id, pos, len) = files[i];
        for (j, &(s_pos, s_len)) in spaces.iter().enumerate() {
            if s_len >= len {
                files[i] = (id, s_pos, len);
                spaces[j] = (s_pos + len, s_len - len);
                break;
            }
            if s_pos >= pos {
                break;
            }
        }
    }

    get_checksum(&files).to_string()
}
