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

    compacted_files
        .into_iter()
        .map(|(id, pos, len)| id * (len * pos + (len - 1) * len / 2))
        .sum::<u64>()
        .to_string()
}

#[allow(unused_variables)]
pub fn part_2(input: &str) -> String {
    String::new()
}
