use std::{
    cell::RefCell,
    cmp::max,
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use common::{Direction, Position};

#[derive(Debug, PartialEq)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

impl Robot {
    fn parse(raw_data: String) -> Self {
        let data: Vec<&str> = raw_data.split(" v=").collect();

        let pos_data: Vec<isize> = data[0]
            .replace("p=", "")
            .split(',')
            .map(|x| x.parse().expect("Invalid number"))
            .collect();

        let vel_data: Vec<i32> = data[1]
            .split(',')
            .map(|x| x.parse().expect("Invalid number"))
            .collect();

        Self {
            pos: Position {
                x: pos_data[0],
                y: pos_data[1],
            },
            vel: Velocity {
                x: vel_data[0],
                y: vel_data[1],
            },
        }
    }

    fn step(&mut self) {
        self.pos.x += self.vel.x as isize;
        self.pos.y += self.vel.y as isize;
    }

    fn teleport(&mut self, pos: Position) {
        self.pos = pos;
    }
}

#[derive(Debug, PartialEq)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct RoomSecurity {
    robots: RefCell<Vec<Robot>>,
    room_size: Size,
}

impl RoomSecurity {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let robots = reader
            .lines()
            .map(|line| Robot::parse(line.unwrap()))
            .collect();

        Self {
            robots: RefCell::new(robots),
            room_size: Size { x: 101, y: 103 },
        }
    }

    fn step(&self) {
        for robot in self.robots.borrow_mut().iter_mut() {
            robot.step();

            if self.is_within_bounds(&robot.pos) {
                continue;
            }

            let new_pos = self.calculate_teleport_pos(&robot.pos);

            robot.teleport(new_pos);
        }
    }

    fn calculate_safety_factor(&self) -> u64 {
        let mut quadrant_a = 0;
        let mut quadrant_b = 0;
        let mut quadrant_c = 0;
        let mut quadrant_d = 0;

        for robot in self.robots.borrow().iter() {
            // Robots in exactly the middle of the quadrant don't count
            if robot.pos.x as usize == self.room_size.x / 2
                || robot.pos.y as usize == self.room_size.y / 2
            {
                continue;
            }

            if robot.pos.x as usize <= self.room_size.x / 2 {
                if robot.pos.y as usize <= self.room_size.y / 2 {
                    quadrant_a += 1;
                } else {
                    quadrant_c += 1;
                }
            } else if robot.pos.y as usize <= self.room_size.y / 2 {
                quadrant_b += 1;
            } else {
                quadrant_d += 1;
            };
        }

        quadrant_a * quadrant_b * quadrant_c * quadrant_d
    }

    fn calculate_teleport_pos(&self, pos: &Position) -> Position {
        let new_x = self.get_circular_index(pos.x, self.room_size.x);
        let new_y = self.get_circular_index(pos.y, self.room_size.y);

        Position { x: new_x, y: new_y }
    }

    fn get_circular_index(&self, index: isize, limit: usize) -> isize {
        if index < 0 {
            limit as isize - index.abs()
        } else if index > (limit - 1) as isize {
            index - limit as isize
        } else {
            index
        }
    }

    fn is_within_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0
            && pos.x < self.room_size.x as isize
            && pos.y >= 0
            && pos.y < self.room_size.y as isize
    }

    fn generate_matrix(&self) -> Vec<Vec<&str>> {
        let mut matrix = Vec::new();

        for _ in 0..self.room_size.y {
            let row = vec!["."; self.room_size.x];
            matrix.push(row);
        }

        for robot in self.robots.borrow().iter() {
            matrix[robot.pos.y as usize][robot.pos.x as usize] = "#";
        }

        matrix
    }

    fn find_max_connectivity_step(&self) -> u32 {
        // We'll assume that a lot of robots connected together will only occur
        // when actually drawing something, in this case a christmas tree
        let mut max_connectivity = 0;
        let mut step = 0;

        // The top limit might need to be tweaked
        (1..10000).for_each(|i| {
            self.step();

            let connectivity = self.get_highest_connectivity();

            if connectivity > max_connectivity {
                max_connectivity = connectivity;
                step = i;
            }
        });

        step
    }

    fn get_highest_connectivity(&self) -> u32 {
        let mut connectivity: u32 = 0;

        let matrix = self.generate_matrix();
        let directions = Direction::generate_directions_list();

        let mut visited_robots = HashSet::new();

        for robot in self.robots.borrow().iter() {
            if visited_robots.contains(&robot.pos) {
                continue;
            }

            visited_robots.insert(robot.pos.clone());

            let mut queue = vec![robot.pos.clone()];

            while let Some(pos) = queue.pop() {
                for direction in directions.iter() {
                    let new_pos = Direction::apply_offset(direction, pos.x, pos.y);

                    if !self.is_within_bounds(&new_pos) {
                        continue;
                    }

                    if visited_robots.contains(&new_pos) {
                        continue;
                    }

                    if matrix[new_pos.y as usize][new_pos.x as usize] == "#" {
                        queue.push(new_pos);
                    }
                }

                visited_robots.insert(pos);
            }

            connectivity = max(connectivity, visited_robots.len() as u32)
        }

        connectivity
    }
}

impl Display for RoomSecurity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let matrix = self.generate_matrix();

        let matrix_str = matrix
            .iter()
            .map(|row| row.join("").to_string())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", matrix_str)
    }
}

#[derive(Debug, PartialEq)]
struct Size {
    x: usize,
    y: usize,
}

fn main() {
    let room_security = RoomSecurity::parse("day14/data/input.txt");

    (0..100).for_each(|_| room_security.step());

    println!("Part 1 result: {}", room_security.calculate_safety_factor());

    let room_security = RoomSecurity::parse("day14/data/input.txt");

    println!(
        "Part 2 result: {}",
        room_security.find_max_connectivity_step()
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> RoomSecurity {
        RoomSecurity {
            robots: RefCell::new(vec![
                Robot {
                    pos: Position { x: 0, y: 4 },
                    vel: Velocity { x: 3, y: -3 },
                },
                Robot {
                    pos: Position { x: 6, y: 3 },
                    vel: Velocity { x: -1, y: -3 },
                },
                Robot {
                    pos: Position { x: 10, y: 3 },
                    vel: Velocity { x: -1, y: 2 },
                },
                Robot {
                    pos: Position { x: 2, y: 0 },
                    vel: Velocity { x: 2, y: -1 },
                },
                Robot {
                    pos: Position { x: 0, y: 0 },
                    vel: Velocity { x: 1, y: 3 },
                },
                Robot {
                    pos: Position { x: 3, y: 0 },
                    vel: Velocity { x: -2, y: -2 },
                },
                Robot {
                    pos: Position { x: 7, y: 6 },
                    vel: Velocity { x: -1, y: -3 },
                },
                Robot {
                    pos: Position { x: 3, y: 0 },
                    vel: Velocity { x: -1, y: -2 },
                },
                Robot {
                    pos: Position { x: 9, y: 3 },
                    vel: Velocity { x: 2, y: 3 },
                },
                Robot {
                    pos: Position { x: 7, y: 3 },
                    vel: Velocity { x: -1, y: 2 },
                },
                Robot {
                    pos: Position { x: 2, y: 4 },
                    vel: Velocity { x: 2, y: -3 },
                },
                Robot {
                    pos: Position { x: 9, y: 5 },
                    vel: Velocity { x: -3, y: -3 },
                },
            ]),
            room_size: Size { x: 11, y: 7 },
        }
    }

    #[test]
    fn check_parsing() {
        let mut input = RoomSecurity::parse("data/test.txt");
        input.room_size = Size { x: 11, y: 7 };

        let test_data = get_test_input();

        assert_eq!(input, test_data);
    }

    #[test]
    fn check_safety_factor() {
        let test_data = get_test_input();

        (0..100).for_each(|_| test_data.step());

        assert_eq!(test_data.calculate_safety_factor(), 12);
    }
}
