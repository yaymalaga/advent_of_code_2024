use common::Direction;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct WordsBoard {
    data: Vec<Vec<char>>,
}

impl WordsBoard {
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

    fn get_xmas_pattern_count(&self) -> usize {
        let mut matches_count = 0;

        let search_directions = Direction::generate_directions_list();
        let search_pattern = ['M', 'A', 'S'];

        for y in 0..self.data.len() {
            for x in 0..self.data[0].len() {
                if self.data[y][x] != 'X' {
                    continue;
                }

                'direction: for direction in search_directions.iter() {
                    let direction_offset = Direction::get_offset(direction);

                    for (i, &expected_letter) in search_pattern.iter().enumerate() {
                        let new_x = x as isize + direction_offset.x * (i as isize + 1);
                        let new_y = y as isize + direction_offset.y * (i as isize + 1);

                        if !self.is_within_bounds(new_x, new_y) {
                            continue 'direction;
                        }

                        if self.data[new_y as usize][new_x as usize] != expected_letter {
                            continue 'direction;
                        }
                    }

                    matches_count += 1;
                }
            }
        }

        matches_count
    }

    fn get_x_mas_pattern_count(&self) -> usize {
        let mut matches_count = 0;

        for y in 0..self.data.len() {
            'letter: for x in 0..self.data[0].len() {
                if self.data[y][x] != 'A' {
                    continue;
                }

                let diagonals_coordinates = [
                    [
                        Direction::apply_offset(&Direction::TopLeft, x as isize, y as isize),
                        Direction::apply_offset(&Direction::BottomRight, x as isize, y as isize),
                    ],
                    [
                        Direction::apply_offset(&Direction::TopRight, x as isize, y as isize),
                        Direction::apply_offset(&Direction::BottomLeft, x as isize, y as isize),
                    ],
                ];

                for coordinate in diagonals_coordinates.iter().flatten() {
                    if !self.is_within_bounds(coordinate.x, coordinate.y) {
                        continue 'letter;
                    }
                }

                for diagonal_coordinates in diagonals_coordinates {
                    let letter_a = self.data[diagonal_coordinates[0].y as usize]
                        [diagonal_coordinates[0].x as usize];

                    let letter_b = self.data[diagonal_coordinates[1].y as usize]
                        [diagonal_coordinates[1].x as usize];

                    match (letter_a, letter_b) {
                        ('M', 'S') | ('S', 'M') => {}
                        _ => continue 'letter,
                    }
                }

                matches_count += 1;
            }
        }

        matches_count
    }

    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.data[0].len() as isize && y >= 0 && y < self.data.len() as isize
    }
}

fn main() {
    let words_board = WordsBoard::parse("day4/data/input.txt");

    let xmas_pattern_count = words_board.get_xmas_pattern_count();

    println!("Part 1 result: {}", xmas_pattern_count);

    let x_mas_pattern_count = words_board.get_x_mas_pattern_count();

    println!("Part 2 result: {}", x_mas_pattern_count);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> WordsBoard {
        WordsBoard {
            data: vec![
                vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
            ],
        }
    }

    #[test]
    fn check_parsing() {
        let input = WordsBoard::parse("data/test1.txt");

        let test_data = get_test_input();

        for j in 0..input.data.len() {
            for i in 0..input.data[0].len() {
                assert_eq!(input.data[j][i], test_data.data[j][i]);
            }
        }
    }

    #[test]
    fn check_xmas_pattern_count() {
        let test_data = get_test_input();

        assert_eq!(test_data.get_xmas_pattern_count(), 18);
    }

    #[test]
    fn check_x_mas_pattern_count() {
        let test_data = WordsBoard::parse("data/test2.txt");

        assert_eq!(test_data.get_x_mas_pattern_count(), 9);
    }
}
