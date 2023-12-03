fn main() {
    let input = include_str!("../input.txt");

    println!("part1 = {}", part1(input));
    println!("part2 = {}", part2(input));
}

fn parse_game_line(line: &str) -> (usize, Vec<(usize, usize, usize)>) {
    let (game_name, hands) = line.split_once(": ").unwrap();

    (
        game_name.split_once(" ").unwrap().1.parse::<_>().unwrap(),
        hands
            .split("; ")
            .map(|hand| {
                hand.split(", ").fold((0, 0, 0), |counts, hand| {
                    let (count, color) = hand.split_once(" ").unwrap();
                    let count = count.parse::<_>().unwrap();
                    match color {
                        "red" => (count, counts.1, counts.2),
                        "green" => (counts.0, count, counts.2),
                        "blue" => (counts.0, counts.1, count),
                        _ => panic!("invalid color {color}"),
                    }
                })
            })
            .collect::<Vec<_>>(),
    )
}

fn part1(input: &str) -> usize {
    let max_cubes = (12, 13, 14);

    input
        .lines()
        .map(parse_game_line)
        .filter_map(|game| {
            let game_max = game
                .1
                .into_iter()
                .reduce(|acc, game| (acc.0.max(game.0), acc.1.max(game.1), acc.2.max(game.2)))
                .unwrap();

            if game_max.0 <= max_cubes.0 && game_max.1 <= max_cubes.1 && game_max.2 <= max_cubes.2 {
                Some(game.0)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_game_line)
        .map(|game| {
            let game_min = game
                .1
                .into_iter()
                .reduce(|acc, game| (acc.0.max(game.0), acc.1.max(game.1), acc.2.max(game.2)))
                .unwrap();

            game_min.0 * game_min.1 * game_min.2
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 2286);
    }
}
