fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let total = part1(&input);
    println!("Total part 1: {}", total);
    let total = part2(&input);
    println!("Total part 2: {}", total);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut first_digit = None;
            let mut last_digit = None;
            for c in line.chars() {
                if let Some(d) = c.to_digit(10) {
                    if first_digit.is_none() {
                        first_digit = Some(d);
                    }
                    last_digit = Some(d);
                }
            }
            first_digit.unwrap() * 10 + last_digit.unwrap()
        })
        .sum()
}

fn digits(line: &str) -> Vec<u32> {
    let mut i = 0;
    let mut digits: Vec<u32> = Vec::new();
    while i < line.len() {
        if line.chars().nth(i).unwrap() == 'o' && i + 2 < line.len() && &line[i..i + 3] == "one" {
            digits.push(1);
        } else if line.chars().nth(i).unwrap() == 'e'
            && i + 4 < line.len()
            && &line[i..i + 5] == "eight"
        {
            digits.push(8);
        } else if line.chars().nth(i).unwrap() == 't'
            && i + 2 < line.len()
            && &line[i..i + 3] == "two"
        {
            digits.push(2);
        } else if line.chars().nth(i).unwrap() == 't'
            && i + 4 < line.len()
            && &line[i..i + 5] == "three"
        {
            digits.push(3);
        } else if line.chars().nth(i).unwrap() == 'f'
            && i + 3 < line.len()
            && &line[i..i + 4] == "four"
        {
            digits.push(4);
        } else if line.chars().nth(i).unwrap() == 'f'
            && i + 3 < line.len()
            && &line[i..i + 4] == "five"
        {
            digits.push(5);
        } else if line.chars().nth(i).unwrap() == 's'
            && i + 2 < line.len()
            && &line[i..i + 3] == "six"
        {
            digits.push(6);
        } else if line.chars().nth(i).unwrap() == 's'
            && i + 4 < line.len()
            && &line[i..i + 5] == "seven"
        {
            digits.push(7);
        } else if line.chars().nth(i).unwrap() == 'n'
            && i + 3 < line.len()
            && &line[i..i + 4] == "nine"
        {
            digits.push(9);
        } else if line.chars().nth(i).unwrap().is_ascii_digit() {
            digits.push(line.chars().nth(i).unwrap().to_digit(10).unwrap());
        }
        i += 1;
    }
    digits
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|orig_line| {
            let digits = digits(orig_line);
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let total = part1(input);
        assert_eq!(total, 142);
    }

    #[test]
    fn test_example_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let total = part2(input);
        assert_eq!(total, 281);
    }

    #[test]
    fn test_to_digits() {
        let digits = super::digits("two1nine");
        assert_eq!(digits, vec![2, 1, 9]);

        let digits = super::digits("eightwothree");
        assert_eq!(digits, vec![8, 2, 3]);

        let digits = super::digits("abcone2threexyz");
        assert_eq!(digits, vec![1, 2, 3]);

        let digits = super::digits("xtwone3four");
        assert_eq!(digits, vec![2, 1, 3, 4]);

        let digits = super::digits("4nineeightseven2");
        assert_eq!(digits, vec![4, 9, 8, 7, 2]);

        let digits = super::digits("zoneight234");
        assert_eq!(digits, vec![1, 8, 2, 3, 4]);

        let digits = super::digits("7pqrstsixteen");
        assert_eq!(digits, vec![7, 6]);

        let digits = super::digits("zerozerozero");
        assert_eq!(digits, vec![]);

        let digits = super::digits("threerznlrhtkjp23mtflmbrzq395three");
        assert_eq!(digits, vec![3, 2, 3, 3, 9, 5, 3]);

        let digits = super::digits("9sevenvlttm");
        assert_eq!(digits, vec![9, 7]);

        let digits = super::digits("3twochzbv");
        assert_eq!(digits, vec![3, 2]);

        let digits = super::digits("mdxdlh5six5nqfld9bqzxdqxfour");
        assert_eq!(digits, vec![5, 6, 5, 9, 4]);

        let digits = super::digits("422268");
        assert_eq!(digits, vec![4, 2, 2, 2, 6, 8]);

        let digits = super::digits("vdctljvnj2jpgdfnbpfjv1");
        assert_eq!(digits, vec![2, 1]);

        let digits = super::digits("tshl7foureightvzvzdcgt");
        assert_eq!(digits, vec![7, 4, 8]);

        let digits = super::digits("1fourrj");
        assert_eq!(digits, vec![1, 4]);

        let digits = super::digits("6mfbqtzbprqfive");
        assert_eq!(digits, vec![6, 5]);

        let digits = super::digits("4sevens34");
        assert_eq!(digits, vec![4, 7, 3, 4]);

        let digits = super::digits("gdgj3f");
        assert_eq!(digits, vec![3]);

        let digits = super::digits("knqxmrrmninegr4");
        assert_eq!(digits, vec![9, 4]);

        let digits = super::digits("r4");
        assert_eq!(digits, vec![4]);

        let digits = super::digits("1v");
        assert_eq!(digits, vec![1]);

        let digits = super::digits("375threethree");
        assert_eq!(digits, vec![3, 7, 5, 3, 3]);
    }
}
