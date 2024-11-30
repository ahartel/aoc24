fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let ids = part1(&input);
    println!("Part 1: {}", ids);
    let power_sum = part2(&input);
    println!("Part 2: {}", power_sum);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(id_and_max_scores)
        .filter_map(|(id, scores)| {
            (scores[0] <= 12 && scores[1] <= 13 && scores[2] <= 14).then_some(id)
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(id_and_max_scores)
        .map(|(_, scores)| scores[0] * scores[1] * scores[2])
        .sum()
}

fn id_and_max_scores(game: &str) -> (u32, Vec<u32>) {
    let split: Vec<_> = game.split(':').collect();
    let results: Vec<_> = split[1].split(';').collect();
    let mut scores = vec![0; 3];
    for result in results {
        let colors: Vec<_> = result.split(", ").collect();
        for color in colors {
            let split: Vec<_> = color.trim().split(' ').collect();
            let count = split[0].parse::<u32>().unwrap();
            if color.contains("red") && count > scores[0] {
                scores[0] = count;
            } else if color.contains("green") && count > scores[1] {
                scores[1] = count;
            } else if color.contains("blue") && count > scores[2] {
                scores[2] = count;
            }
        }
    }
    let id = split[0].split(' ').nth(1).unwrap().parse::<u32>().unwrap();
    (id, scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let ids = part1(games);
        assert_eq!(ids, 8);
    }

    #[test]
    fn example_part_2() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let power_sum = part2(games);
        assert_eq!(power_sum, 2286);
    }

    #[test]
    fn can_parse_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (id, scores) = id_and_max_scores(game);
        assert_eq!(id, 1);
        assert_eq!(scores, vec![4, 2, 6]);
    }
}
