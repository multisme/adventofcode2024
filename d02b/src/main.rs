fn is_tolerantly_decreasing(report: &[i32], depth: u8) -> bool {
    let size = report.len();

    if size <= 1 {
        return true;
    }
    for i in 0..size - 1 {
        if report[i] <= report[i + 1] || report[i] - report[i + 1] > 3 {
            if depth > 0 {
                return false;
            }
            let mut slice1 = report.to_vec();
            let mut slice2 = slice1.clone();
            slice1.remove(i);
            slice2.remove(i + 1);
            return is_tolerantly_decreasing(&slice1, depth + 1)
                || is_tolerantly_decreasing(&slice2, depth + 1);
        }
    }
    true
}

fn is_tolerantly_increasing(report: &[i32], depth: u8) -> bool {
    let size = report.len();
    if size <= 1 {
        return true;
    }
    for i in 0..size - 1 {
        if report[i] >= report[i + 1] || report[i + 1] - report[i] > 3 {
            if depth > 0 {
                return false;
            }
            let mut slice1 = report.to_vec();
            let mut slice2 = slice1.clone();
            slice1.remove(i);
            slice2.remove(i + 1);
            return is_tolerantly_increasing(&slice1, depth + 1)
                || is_tolerantly_increasing(&slice2, depth + 1);
        }
    }
    true
}

fn prepare_report(report: &[i32]) -> bool {
    is_tolerantly_increasing(&report, 0) || is_tolerantly_decreasing(&report, 0)
}

fn main() {
    let reports = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_reports = reports
        .clone()
        .into_iter()
        .filter(|report| prepare_report(report))
        .collect::<Vec<_>>();
    println!("{:?}", safe_reports.len());
}
