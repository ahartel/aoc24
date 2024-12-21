fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let safe = input
        .lines()
        .filter_map(|line| is_safe_part_1(line).then_some(()))
        .count();
    println!("Part 1: {}", safe);
    let safe = input
        .lines()
        .filter_map(|line| is_safe_part_2(line).then_some(()))
        .count();
    println!("Part 2: {}", safe);
}

fn safe(levels: &[i32]) -> bool {
    levels
        .iter()
        .fold((None, None, 0), |(prev2, prev1, result), n| {
            match (prev2, prev1, result) {
                (Some(prev2), Some(prev1), fails) => {
                    if (prev1 > prev2 && (n - prev1 == 1 || n - prev1 == 2 || n - prev1 == 3))
                        || (prev1 < prev2
                            && (n - prev1 == -1 || n - prev1 == -2 || n - prev1 == -3))
                    {
                        (Some(prev1), Some(n), fails)
                    } else {
                        (Some(prev1), Some(n), fails + 1)
                    }
                }
                (None, None, fails) => (None, Some(n), fails),
                (None, Some(prev1), fails) => {
                    if (n - prev1 == 1 || n - prev1 == 2 || n - prev1 == 3)
                        || (n - prev1 == -1 || n - prev1 == -2 || n - prev1 == -3)
                    {
                        (Some(prev1), Some(n), fails)
                    } else {
                        (Some(prev1), Some(n), fails + 1)
                    }
                }
                _ => (prev2, prev1, result),
            }
        })
        .2
        == 0
}

fn is_safe_part_1(report: &str) -> bool {
    safe(
        &report
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>(),
    )
}

fn is_safe_part_2(report: &str) -> bool {
    let numbers: Vec<_> = report
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    for skip in 0..numbers.len() {
        let mut to_check = numbers.clone();
        to_check.remove(skip);
        if safe(&to_check) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let example = "7 6 4 2 1";
        assert!(is_safe_part_1(example));
        assert!(is_safe_part_2(example));
    }

    #[test]
    fn second_example() {
        let example = "1 2 7 8 9";
        assert!(!is_safe_part_1(example));
        assert!(!is_safe_part_2(example));
    }

    #[test]
    fn third_example() {
        let example = "9 7 6 2 1";
        assert!(!is_safe_part_1(example));
        assert!(!is_safe_part_2(example));
    }

    #[test]
    fn fourth_example() {
        let example = "1 3 2 4 5";
        assert!(!is_safe_part_1(example));
        assert!(is_safe_part_2(example));
    }

    #[test]
    fn fifth_example() {
        let example = "8 6 4 4 1";
        assert!(!is_safe_part_1(example));
        assert!(is_safe_part_2(example));
    }

    #[test]
    fn sixth_example() {
        let example = "1 3 6 7 9";
        assert!(is_safe_part_1(example));
        assert!(is_safe_part_2(example));
    }
}
