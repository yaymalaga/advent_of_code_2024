use common::Position;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
struct AntennasMap {
    data: Vec<Vec<char>>,
}

impl AntennasMap {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let mut words_board_data = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();

            let line_data: Vec<char> = line.chars().collect();

            words_board_data.push(line_data);
        }

        Self {
            data: words_board_data,
        }
    }

    fn get_antennas_grouped_by_frequency(&self) -> HashMap<char, Vec<Position>> {
        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

        for (y, line) in self.data.iter().enumerate() {
            for (x, &frequency) in line.iter().enumerate() {
                if frequency == '.' {
                    continue;
                }

                antennas
                    .entry(frequency)
                    .and_modify(|vec| {
                        vec.push(Position {
                            x: x as isize,
                            y: y as isize,
                        })
                    })
                    .or_insert(vec![Position {
                        x: x as isize,
                        y: y as isize,
                    }]);
            }
        }

        antennas
    }

    fn get_unique_antinodes_count(&self, use_resonant_harmonics: bool) -> usize {
        let mut antinodes: HashSet<Position> = HashSet::new();

        let antennas_groups = self.get_antennas_grouped_by_frequency();

        for (_, antennas) in antennas_groups.iter() {
            // Compare each antenna againt the others
            for current_antenna in antennas {
                for next_antenna in antennas {
                    // Don't compare an antenna against itself
                    if current_antenna == next_antenna {
                        continue;
                    }

                    // Get generated antinodes
                    let calculated_antinodes = self.calculate_antinodes(
                        current_antenna,
                        next_antenna,
                        use_resonant_harmonics,
                    );

                    // Try to add the antinode
                    for antinode in calculated_antinodes {
                        antinodes.insert(antinode);
                    }
                }
            }
        }

        antinodes.len()
    }

    fn calculate_antinodes(
        &self,
        base_antenna: &Position,
        next_antenna: &Position,
        use_resonant_harmonics: bool,
    ) -> Vec<Position> {
        let mut antinodes = Vec::new();

        let delta_x = base_antenna.x - next_antenna.x;
        let delta_y = base_antenna.y - next_antenna.y;

        // Antinodes are located twice the distance difference, then three times, and so on
        let mut multiplier = 2;
        loop {
            let antinode = Position {
                x: base_antenna.x - (delta_x * multiplier),
                y: base_antenna.y - (delta_y * multiplier),
            };

            // Stop once the antinode is out of bounds
            if !self.is_within_bounds(antinode.x, antinode.y) {
                break;
            }

            // Otherwise, add it to the list
            antinodes.push(antinode.clone());

            // When not using resonant harmonics, only one antinode is generated
            if !use_resonant_harmonics {
                break;
            }

            multiplier += 1;
        }

        // When using harmonics, an antinode is also generated at the base antenna
        if use_resonant_harmonics {
            antinodes.push(base_antenna.clone());
        }

        antinodes
    }

    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.data[0].len() as isize && y >= 0 && y < self.data.len() as isize
    }
}

fn main() {
    let antennas_map = AntennasMap::parse("day8/data/input.txt");

    let unique_antinodes = antennas_map.get_unique_antinodes_count(false);

    println!("Part 1 result: {}", unique_antinodes);

    let unique_antinodes = antennas_map.get_unique_antinodes_count(true);

    println!("Part 2 result: {}", unique_antinodes);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> AntennasMap {
        AntennasMap {
            data: vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', 'A', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ],
        }
    }

    #[test]
    fn check_parsing() {
        let input = AntennasMap::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input, test_data);
    }

    #[test]
    fn check_unique_antinodes_count() {
        let test_data = get_test_input();

        assert_eq!(test_data.get_unique_antinodes_count(false), 14);
    }

    #[test]
    fn check_harmonics_unique_antinodes_count() {
        let test_data = get_test_input();

        assert_eq!(test_data.get_unique_antinodes_count(true), 34);
    }
}
