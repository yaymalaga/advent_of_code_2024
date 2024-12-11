use common::{Direction, Position};

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
struct TopographicMap {
    data: Vec<Vec<u32>>,
}

impl TopographicMap {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let data: Vec<Vec<u32>> = reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .trim()
                    .chars()
                    .map(|x| x.to_digit(10).expect("Can't convert char to digit"))
                    .collect()
            })
            .collect();

        Self { data }
    }

    fn find_trailheads(&self) -> Vec<Position> {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, item)| **item == 0)
                    .map(|(x, _)| Position {
                        x: x as isize,
                        y: y as isize,
                    })
                    .collect::<Vec<Position>>()
            })
            .collect()
    }

    fn get_trailheads_tops(&self, trailhead: &Position) -> Vec<Position> {
        let directions = Direction::generate_basic_directions_list();

        // The same top can be reached from several paths
        let mut trailhead_tops = Vec::new();

        // Get a position, generate possible new ones, queue if valid, and start again
        let mut queued_next_pos = vec![trailhead.clone()];
        while let Some(current_pos) = queued_next_pos.pop() {
            let current_pos_value = self.data[current_pos.y as usize][current_pos.x as usize];

            for direction in directions.iter() {
                let next_pos = Direction::apply_offset(direction, current_pos.x, current_pos.y);

                if !self.is_within_bounds(next_pos.x, next_pos.y) {
                    continue;
                }

                let next_pos_value = self.data[next_pos.y as usize][next_pos.x as usize];

                // Needs to be increasing one by one
                if next_pos_value != current_pos_value + 1 {
                    continue;
                }

                if next_pos_value == 9 {
                    trailhead_tops.push(next_pos.clone());
                } else {
                    queued_next_pos.push(next_pos.clone());
                }
            }
        }

        trailhead_tops
    }

    fn calculate_trailheads_total_score(&self) -> u32 {
        self.find_trailheads()
            .iter()
            .map(|trailhead| {
                let mut unique_trailheads = HashSet::new();

                self.get_trailheads_tops(trailhead)
                    .into_iter()
                    .for_each(|top| {
                        unique_trailheads.insert(top);
                    });

                unique_trailheads.len() as u32
            })
            .sum()
    }

    fn calculate_trailheads_total_rating(&self) -> u32 {
        self.find_trailheads()
            .iter()
            .map(|trailhead| self.get_trailheads_tops(trailhead).len() as u32)
            .sum()
    }

    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.data[0].len() as isize && y >= 0 && y < self.data.len() as isize
    }
}

fn main() {
    let topographic_map = TopographicMap::parse("day10/data/input.txt");

    let trailheads_score = topographic_map.calculate_trailheads_total_score();

    println!("Part 1 result: {}", trailheads_score);

    let trailheads_rating = topographic_map.calculate_trailheads_total_rating();

    println!("Part 2 result: {}", trailheads_rating);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> TopographicMap {
        TopographicMap {
            data: vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3],
                vec![7, 8, 1, 2, 1, 8, 7, 4],
                vec![8, 7, 4, 3, 0, 9, 6, 5],
                vec![9, 6, 5, 4, 9, 8, 7, 4],
                vec![4, 5, 6, 7, 8, 9, 0, 3],
                vec![3, 2, 0, 1, 9, 0, 1, 2],
                vec![0, 1, 3, 2, 9, 8, 0, 1],
                vec![1, 0, 4, 5, 6, 7, 3, 2],
            ],
        }
    }

    #[test]
    fn check_parsing() {
        let input = TopographicMap::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input, test_data);
    }

    #[test]
    fn check_trailheads_total_score() {
        let test_data = get_test_input();

        assert_eq!(test_data.calculate_trailheads_total_score(), 36);
    }

    #[test]
    fn check_trailheads_total_rating() {
        let test_data = get_test_input();

        assert_eq!(test_data.calculate_trailheads_total_rating(), 81);
    }
}
