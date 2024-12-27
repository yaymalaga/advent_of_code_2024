use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use common::{Offset, Position};

#[derive(Debug, Clone, PartialEq)]
struct ClawMachine {
    button_a_cost: usize,
    button_a_offset: Offset,
    button_b_cost: usize,
    button_b_offset: Offset,
    prize_pos: Position,
}

impl ClawMachine {
    fn get_prize(&self, prize_offset: Option<usize>) -> Option<usize> {
        let prize_pos = Position {
            x: self.prize_pos.x + prize_offset.unwrap_or_default() as isize,
            y: self.prize_pos.y + prize_offset.unwrap_or_default() as isize,
        };

        // The claw machine is actually a two system linear equation,
        // one for the x axis and another one for the y axis
        // We need to find their intersection point if any, by using
        // the already developed equation below
        let b = ((prize_pos.y * self.button_a_offset.x) - (self.button_a_offset.y * prize_pos.x))
            as f64
            / ((-self.button_a_offset.y * self.button_b_offset.x)
                + (self.button_a_offset.x * self.button_b_offset.y)) as f64;

        let a = (prize_pos.x - self.button_b_offset.x * b as isize) as f64
            / self.button_a_offset.x as f64;

        // Accept only whole numbers
        if b.trunc() != b || a.trunc() != a {
            return None;
        }

        Some(self.button_a_cost * a as usize + self.button_b_cost * b as usize)
    }
}

#[derive(Debug, PartialEq)]
struct Arcade {
    games: Vec<ClawMachine>,
}

impl Arcade {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let mut claw_machines = Vec::new();

        let mut claw_machine = ClawMachine {
            button_a_cost: 3,
            button_a_offset: Offset { x: 0, y: 0 },
            button_b_cost: 1,
            button_b_offset: Offset { x: 0, y: 0 },
            prize_pos: Position { x: 0, y: 0 },
        };

        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();

            if line.is_empty() {
                continue;
            }

            let line_data: Vec<&str> = line.split(":").collect();

            let offset_data: Vec<isize> = line_data[1]
                .split(",")
                .map(|item| {
                    let item = item.trim();
                    let item = item.replace("X", "");
                    let item = item.replace("Y", "");
                    let item = item.replace("+", "");
                    let item = item.replace("=", "");
                    item.parse::<isize>().expect("Can't convert to number")
                })
                .take(2)
                .collect();

            match line_data[0] {
                "Button A" => {
                    claw_machine.button_a_offset = Offset {
                        x: offset_data[0],
                        y: offset_data[1],
                    }
                }
                "Button B" => {
                    claw_machine.button_b_offset = Offset {
                        x: offset_data[0],
                        y: offset_data[1],
                    }
                }
                "Prize" => {
                    claw_machine.prize_pos = Position {
                        x: offset_data[0],
                        y: offset_data[1],
                    };

                    claw_machines.push(claw_machine.clone());
                }
                _ => panic!("Unexpected string"),
            };
        }

        Self {
            games: claw_machines,
        }
    }

    fn get_total_cost(&self, prize_offset: Option<usize>) -> usize {
        self.games
            .iter()
            .flat_map(|game| game.get_prize(prize_offset))
            .sum()
    }
}

fn main() {
    let arcade = Arcade::parse("day13/data/input.txt");

    println!("Part 1 result: {}", arcade.get_total_cost(None));

    println!(
        "Part 2 result: {}",
        arcade.get_total_cost(Some(10000000000000))
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> Arcade {
        Arcade {
            games: vec![
                ClawMachine {
                    button_a_cost: 3,
                    button_a_offset: Offset { x: 94, y: 34 },
                    button_b_cost: 1,
                    button_b_offset: Offset { x: 22, y: 67 },
                    prize_pos: Position { x: 8400, y: 5400 },
                },
                ClawMachine {
                    button_a_cost: 3,
                    button_a_offset: Offset { x: 26, y: 66 },
                    button_b_cost: 1,
                    button_b_offset: Offset { x: 67, y: 21 },
                    prize_pos: Position { x: 12748, y: 12176 },
                },
                ClawMachine {
                    button_a_cost: 3,
                    button_a_offset: Offset { x: 17, y: 86 },
                    button_b_cost: 1,
                    button_b_offset: Offset { x: 84, y: 37 },
                    prize_pos: Position { x: 7870, y: 6450 },
                },
                ClawMachine {
                    button_a_cost: 3,
                    button_a_offset: Offset { x: 69, y: 23 },
                    button_b_cost: 1,
                    button_b_offset: Offset { x: 27, y: 71 },
                    prize_pos: Position { x: 18641, y: 10279 },
                },
            ],
        }
    }

    #[test]
    fn check_parsing() {
        let input = Arcade::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input, test_data);
    }

    #[test]
    fn check_claw_machines_cost() {
        let input = get_test_input();

        assert_eq!(input.games[0].get_prize(None), Some(280));
        assert_eq!(input.games[1].get_prize(None), None);
        assert_eq!(input.games[2].get_prize(None), Some(200));
        assert_eq!(input.games[3].get_prize(None), None);
    }

    #[test]
    fn check_arcade_total_cost() {
        let input = get_test_input();

        assert_eq!(input.get_total_cost(None), 480);
    }

    #[test]
    fn check_offset_claw_machines_cost() {
        let input = get_test_input();

        assert_eq!(input.games[0].get_prize(Some(10000000000000)), None);
        assert_eq!(
            input.games[1].get_prize(Some(10000000000000)),
            Some(459236326669)
        );
        assert_eq!(input.games[2].get_prize(Some(10000000000000)), None);
        assert_eq!(
            input.games[3].get_prize(Some(10000000000000)),
            Some(416082282239)
        );
    }

    #[test]
    fn check_offset_arcade_total_cost() {
        let input = get_test_input();

        assert_eq!(input.get_total_cost(Some(10000000000000)), 875318608908);
    }
}
