fn get_safe_reports() -> i32 {
    let input = include_str!("../puzzle.txt").trim();
    let reports: Vec<&str> = input.split("\n").collect();

    let mut safe_reports = 0;

    for report in reports {
        let levels: Vec<i32> = report.split(" ").map(|x| x.parse().unwrap()).collect();
        let is_safe = is_safe_report(&levels);

        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let differences: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    let all_increasing = differences.iter().all(|&diff| diff > 0);
    let all_decreasing = differences.iter().all(|&diff| diff < 0);

    if !all_increasing && !all_decreasing {
        return false;
    }

    differences
        .iter()
        .all(|&diff| (1..=3).contains(&diff.abs()))
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe_report(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels: Vec<i32> = levels.to_vec();
        modified_levels.remove(i);

        if is_safe_report(&modified_levels) {
            return true;
        }
    }

    false
}

fn count_safe_reports_with_dumpener() -> i32 {
    let input = include_str!("../puzzle.txt").trim();
    let reports: Vec<&str> = input.split("\n").collect();

    let mut safe_reports = 0;

    for report in reports {
        let levels: Vec<i32> = report.split(" ").map(|x| x.parse().unwrap()).collect();
        let is_safe = is_safe_with_dampener(&levels);

        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

//fn count_safe_reports(input: &str) -> usize {
//    input
//        .lines()
//        .filter(|line| !line.is_empty())
//        .map(|line| {
//            line.split_whitespace()
//                .filter_map(|num| num.parse::<i32>().ok())
//                .collect::<Vec<i32>>()
//        })
//        .filter(|levels| is_safe_with_dampener(levels))
//        .count()
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_safe_reports() {
        assert_eq!(get_safe_reports(), 369);
    }

    #[test]
    fn test_count_safe_reports_with_dumpener() {
        assert_eq!(count_safe_reports_with_dumpener(), 428);
    }
}
