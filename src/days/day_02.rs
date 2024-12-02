use crate::util::parse_grid_rows;

fn report_is_valid(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];

        let diff = (a - b).abs();
        if diff == 0 || diff > 3 {
            return false;
        }

        if a < b {
            decreasing = false;
        }
        if a > b {
            increasing = false;
        }
    }
    increasing || decreasing
}

pub fn part_1(input: &str) -> String {
    parse_grid_rows(input)
        .into_iter()
        .filter(|r| report_is_valid(r))
        .count()
        .to_string()
}

fn report_is_valid_with_dampener(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut fixed_report = report.to_owned();
        fixed_report.remove(i);
        if report_is_valid(&fixed_report) {
            return true;
        }
    }
    false
}

pub fn part_2(input: &str) -> String {
    parse_grid_rows(input)
        .into_iter()
        .filter(|r| report_is_valid_with_dampener(r))
        .count()
        .to_string()
}
