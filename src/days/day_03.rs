pub fn part_1(input: &str) -> String {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum::<i32>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let mut clean_input = String::new();
    let mut enabled = true;

    for i in 0..input.len() {
        if enabled {
            // slicing input like this is ok, because its an ascii string
            if input[i..].starts_with("don't()") {
                enabled = false
            } else {
                clean_input.push(input.as_bytes()[i] as char);
            }
        } else if input[i..].starts_with("do()") {
            enabled = true;
        }
    }
    part_1(&clean_input)
}
