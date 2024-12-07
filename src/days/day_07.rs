fn is_good(target: u64, curr_value: u64, nums: &[u64], is_part_2: bool) -> bool {
    if nums.is_empty() {
        return target == curr_value;
    } else if curr_value > target {
        return false;
    }
    let next_num = nums.first().unwrap();

    let concated = curr_value * (10_u64.pow(next_num.ilog10() + 1)) + next_num;

    is_good(target, curr_value + next_num, &nums[1..], is_part_2)
        || is_good(target, curr_value * next_num, &nums[1..], is_part_2)
        || is_part_2 && is_good(target, concated, &nums[1..], is_part_2)
}

fn solve(input: &str, is_part_2: bool) -> String {
    input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            let target = it.next().unwrap().trim_end_matches(':').parse().unwrap();
            let nums = it.map(|s| s.parse().unwrap()).collect::<Vec<_>>();
            (target, nums)
        })
        .filter_map(|(t, n)| is_good(t, *n.first().unwrap(), &n[1..], is_part_2).then(|| t))
        .sum::<u64>()
        .to_string()
}

pub fn part_1(input: &str) -> String {
    solve(input, false)
}

pub fn part_2(input: &str) -> String {
    solve(input, true)
}
