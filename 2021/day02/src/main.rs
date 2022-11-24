fn main() {
    let input = include_str!("../input.txt");

    let ship = dive::<ShipPosition>(input);

    println!("Final position: {:?}", ship);
    println!("Position product: {}", ship.product());

    let ship = dive::<ShipPositionWithAim>(input);

    println!("Final position: {:?}", ship);
    println!("Position product: {}", ship.product());
}

#[derive(Debug, PartialEq)]
enum Command {
    Foward(i64),
    Down(i64),
    Up(i64),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let parts = s.split_once(" ").expect("No space in string.");
        let number: i64 = parts.1.parse().expect("Can't parse amount to integer.");

        match parts.0 {
            "forward" => Command::Foward(number),
            "down" => Command::Down(number),
            "up" => Command::Up(number),
            _ => panic!("Invalid command."),
        }
    }
}

trait Ship {
    fn new() -> Self;
    fn product(&self) -> i64;
    fn run_command(&mut self, cmd: Command);
    fn run_commands(&mut self, cmds: impl Iterator<Item = Command>) {
        for cmd in cmds {
            self.run_command(cmd);
        }
    }
}

#[derive(Debug)]
struct ShipPosition {
    horizontal: i64,
    depth: i64,
}

impl Ship for ShipPosition {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    fn product(&self) -> i64 {
        self.horizontal * self.depth
    }

    fn run_command(&mut self, cmd: Command) {
        match cmd {
            Command::Foward(amount) => self.horizontal += amount,
            Command::Down(amount) => self.depth += amount,
            Command::Up(amount) => self.depth -= amount,
        };
    }
}

#[derive(Debug)]
struct ShipPositionWithAim {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Ship for ShipPositionWithAim {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn product(&self) -> i64 {
        self.horizontal * self.depth
    }

    fn run_command(&mut self, cmd: Command) {
        match cmd {
            Command::Foward(amount) => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
            Command::Down(amount) => self.aim += amount,
            Command::Up(amount) => self.aim -= amount,
        };
    }
}

fn dive<T: Ship>(input: &str) -> T {
    let commands = input.lines().map(|line| Command::from(line));

    let mut ship = T::new();
    ship.run_commands(commands);

    ship
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commands() {
        assert_eq!(Command::from("forward 5"), Command::Foward(5));
        assert_eq!(Command::from("down 2"), Command::Down(2));
        assert_eq!(Command::from("up 3"), Command::Up(3));
    }

    #[test]
    fn test_part1_example() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let ship = dive::<ShipPosition>(input);
        assert_eq!(ship.horizontal, 15);
        assert_eq!(ship.depth, 10);
        assert_eq!(ship.product(), 150);

        let ship = dive::<ShipPositionWithAim>(input);
        assert_eq!(ship.horizontal, 15);
        assert_eq!(ship.depth, 60);
        assert_eq!(ship.product(), 900);
    }
}
