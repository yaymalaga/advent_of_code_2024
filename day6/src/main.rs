use common::{Direction, Position};

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
enum MapTile {
    Empty,
    Obstacle,
    Guard,
}

impl From<char> for MapTile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' => Self::Guard,
            _ => panic!("Unknown tile"),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<MapTile>>,
}

impl Map {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let map_tiles = reader
            .lines()
            .map(|line| line.unwrap().trim().to_string())
            .map(|line| {
                line.chars()
                    .map(|tile| tile.into())
                    .collect::<Vec<MapTile>>()
            })
            .collect();

        Self { tiles: map_tiles }
    }

    fn find_guard_pos(&self) -> Option<Position> {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == MapTile::Guard {
                    return Some(Position {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }

        None
    }

    fn get_guards_route_distinc_tiles(&self) -> Result<HashSet<Position>, ()> {
        let mut distinc_positions = HashSet::new();

        let mut guard_pos = self.find_guard_pos().expect("Guard is not within the map");
        let mut guard_direction = Direction::Top;

        distinc_positions.insert(guard_pos.clone());

        let max_loop_count = 1000; // Might need to be tweaked
        let mut consecutive_loop_count = 0;

        loop {
            // Compute new step position
            let new_pos = Direction::apply_offset(&guard_direction, guard_pos.x, guard_pos.y);

            // Check if guard left the map
            if !self.is_within_bounds(new_pos.x, new_pos.y) {
                break;
            }

            // Check for obstacles and rotate if any
            if self.tiles[new_pos.y as usize][new_pos.x as usize] == MapTile::Obstacle {
                guard_direction = Direction::apply_90_clockwise_rotation(&guard_direction);
                continue;
            }

            // Otherwise proceed with the step and save pos
            guard_pos = new_pos;
            if !distinc_positions.insert(guard_pos.clone()) {
                consecutive_loop_count += 1;

                // Detect loop and break
                if consecutive_loop_count >= max_loop_count {
                    return Err(());
                }
            } else {
                consecutive_loop_count = 0;
            }
        }

        Ok(distinc_positions)
    }

    fn get_guards_route_distinc_tiles_count(&self) -> usize {
        self.get_guards_route_distinc_tiles()
            .expect("Invalid route found")
            .len()
    }

    fn get_loop_path_combinations_count(&mut self) -> usize {
        let mut visited_tiles = self
            .get_guards_route_distinc_tiles()
            .expect("Invalid route found");

        // An obstacle can't be set on the guard's position
        let guard_intial_pos = self.find_guard_pos().expect("Guard not found");
        visited_tiles.remove(&guard_intial_pos);

        // For every tile in the path, add an obstacle and check for loops
        let mut loop_combinations = 0;
        for path_tile in visited_tiles {
            self.tiles[path_tile.y as usize][path_tile.x as usize] = MapTile::Obstacle;

            let route = self.get_guards_route_distinc_tiles();
            if route.is_err() {
                loop_combinations += 1;
            }

            self.tiles[path_tile.y as usize][path_tile.x as usize] = MapTile::Empty;
        }

        loop_combinations
    }

    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.tiles[0].len() as isize && y >= 0 && y < self.tiles.len() as isize
    }
}

fn main() {
    let mut map = Map::parse("day6/data/input.txt");

    let distinct_tiles = map.get_guards_route_distinc_tiles_count();

    println!("Part 1 result: {}", distinct_tiles);

    let loop_combinations = map.get_loop_path_combinations_count();

    println!("Part 2 result: {}", loop_combinations);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> Vec<Vec<char>> {
        vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ]
    }

    #[test]
    fn check_parsing() {
        let input = Map::parse("data/test.txt");

        let test_data = get_test_input();

        for (j, row) in input.tiles.iter().enumerate() {
            for (i, item) in row.iter().enumerate() {
                let tile: MapTile = test_data[j][i].into();
                assert_eq!(*item, tile);
            }
        }

        assert_eq!(input.find_guard_pos(), Some(Position { y: 6, x: 4 }));
    }

    #[test]
    fn check_distinc_tiles_route() {
        let input = Map::parse("data/test.txt");

        let distinct_tiles = input.get_guards_route_distinc_tiles_count();

        assert_eq!(distinct_tiles, 41);
    }

    #[test]
    fn check_loop_obtacle_combinations() {
        let mut input = Map::parse("data/test.txt");

        let loop_combinations = input.get_loop_path_combinations_count();

        assert_eq!(loop_combinations, 6);
    }
}
