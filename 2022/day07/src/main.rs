use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let paths = parse_input(input);
    println!("part1: {}", part1(&paths));
    println!("part2: {}", part2(&paths));
}

const TOTAL_SPACE: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;

#[derive(Debug, PartialEq, Eq)]
enum Command<'a> {
    Cd(&'a str),
    Ls,
}

fn parse_command<'a>(cli: &'a str) -> Command<'a> {
    let mut args = cli.split_ascii_whitespace().skip(1); // first is $

    match args.next() {
        Some("cd") => Command::Cd(args.next().unwrap()),
        Some("ls") => Command::Ls,
        _ => panic!("unknown command"),
    }
}

fn parse_input<'a>(input: &'a str) -> HashMap<String, usize> {
    let mut paths: HashMap<String, usize> = HashMap::new();
    let mut current_path = Vec::<&str>::new();

    for line in input.lines() {
        if line.starts_with('$') {
            let cmd = parse_command(line);
            match cmd {
                Command::Cd(dir) => match dir {
                    "/" => current_path.clear(),
                    ".." => {
                        current_path.pop().unwrap();
                    }
                    new => current_path.push(new),
                },
                Command::Ls => {} // we can just ignore it
            };
        } else {
            if !line.starts_with("dir ") {
                let (size, _filename) = line.split_once(' ').unwrap();
                let size = size.parse::<usize>().unwrap();

                for p in 0..(current_path.len() + 1) {
                    // add the size to all parent directories as well
                    let path = current_path[..p].join("/").to_string();

                    paths.entry(path).and_modify(|s| *s += size).or_insert(size);
                }
            }
        }
    }

    paths
}

fn part1(paths: &HashMap<String, usize>) -> usize {
    paths.values().filter(|s| **s <= 100_000).sum()
}

fn part2(paths: &HashMap<String, usize>) -> usize {
    let mut sizes = paths.values().copied().collect::<Vec<usize>>();

    let total = paths[""];

    sizes.sort();

    sizes
        .iter()
        .copied()
        .find(|s| TOTAL_SPACE - (total - *s) >= REQUIRED_SPACE)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_example_input_part1() {
        let paths = parse_input(EXAMPLE_INPUT);

        assert_eq!(part1(&paths), 95437);
    }

    #[test]
    fn test_example_input_part2() {
        let paths = parse_input(EXAMPLE_INPUT);

        assert_eq!(part2(&paths), 24933642);
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("$ ls"), Command::Ls);
        assert_eq!(parse_command("$ cd .."), Command::Cd(".."));
    }
}
