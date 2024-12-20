fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let safe = input
        .lines()
        .filter_map(|line| is_safe_part_1(line).then_some(()))
        .count();
    println!("Part 1: {}", safe);
}

fn is_safe_part_1(report: &str) -> bool {
    report
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .fold((None, None, Ok(())), |(prev2, prev1, result), n| {
            match (prev2, prev1, result) {
                (Some(prev2), Some(prev1), Ok(())) => {
                    if (prev1 > prev2 && (n - prev1 == 1 || n - prev1 == 2 || n - prev1 == 3))
                        || (prev1 < prev2
                            && (n - prev1 == -1 || n - prev1 == -2 || n - prev1 == -3))
                    {
                        (Some(prev1), Some(n), Ok(()))
                    } else {
                        (Some(prev1), Some(n), Err(()))
                    }
                }
                (None, None, Ok(())) => (None, Some(n), Ok(())),
                (None, Some(prev1), Ok(())) => {
                    if (n - prev1 == 1 || n - prev1 == 2 || n - prev1 == 3)
                        || (n - prev1 == -1 || n - prev1 == -2 || n - prev1 == -3)
                    {
                        (Some(prev1), Some(n), Ok(()))
                    } else {
                        (Some(prev1), Some(n), Err(()))
                    }
                }
                _ => (prev2, prev1, Err(())),
            }
        })
        .2
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example_is_safe_part_1() {
        let example = "7 6 4 2 1";
        let safe: bool = is_safe_part_1(example);
        assert!(safe);
    }

    #[test]
    fn second_example_is_safe_part_1() {
        let example = "1 2 7 8 9";
        let safe: bool = is_safe_part_1(example);
        assert!(!safe);
    }

    #[test]
    fn third_example_is_safe_part_1() {
        let example = "9 7 6 2 1";
        let safe: bool = is_safe_part_1(example);
        assert!(!safe);
    }

    #[test]
    fn fourth_example_is_safe_part_1() {
        let example = "1 3 2 4 5";
        let safe: bool = is_safe_part_1(example);
        assert!(!safe);
    }

    #[test]
    fn fifth_example_is_safe_part_1() {
        let example = "8 6 4 4 1";
        let safe: bool = is_safe_part_1(example);
        assert!(!safe);
    }

    #[test]
    fn sixth_example_is_safe_part_1() {
        let example = "1 3 6 7 9";
        let safe: bool = is_safe_part_1(example);
        assert!(safe);
    }
}
