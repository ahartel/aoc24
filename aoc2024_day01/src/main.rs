use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (left, right) = unzip_input(&input);
    let sum_of_diffs = sum_of_diffs(left.clone(), right.clone());
    println!("Part 1: {}", sum_of_diffs);
    println!("Part 2: {}", part_2(&left, &right));
}

fn unzip_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let a: u32 = iter.next().unwrap().parse().unwrap();
            let b: u32 = iter.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip()
}

fn sum_of_diffs(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| if b >= a { b - a } else { a - b })
        .sum()
}

fn part_2(left: &[u32], right: &[u32]) -> u32 {
    let occurrences = right.iter().fold(HashMap::new(), |mut acc, &x| {
        let entry = acc.entry(x).or_insert(0);
        *entry += 1;
        acc
    });
    left.iter()
        .map(|x| occurrences.get(x).unwrap_or(&0) * x)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test1() {
        let (left, right) = unzip_input(EXAMPLE);
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test2() {
        let (left, right) = unzip_input(EXAMPLE);
        let sum_of_diffs = sum_of_diffs(left, right);
        assert_eq!(sum_of_diffs, 11);
    }

    #[test]
    fn test_sum_of_diffs() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 0, 9, 3];
        let sum_of_diffs = sum_of_diffs(left, right);
        assert_eq!(sum_of_diffs, 10);
    }

    #[test]
    fn test_part2() {
        let (left, right) = unzip_input(EXAMPLE);
        let similarity = part_2(&left, &right);
        assert_eq!(similarity, 31);
    }
}
