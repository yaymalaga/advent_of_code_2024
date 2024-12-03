use std::{char, fs::File, io::Read};

struct Memory {
    data: String,
}

impl Memory {
    fn parse(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("File can't be read");

        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to dump into string");
        data = data.trim().to_string();

        Self { data }
    }

    // No bounding checks, we were lucky with the input...
    fn get_valid_multiplications(&self, skip: bool) -> Vec<Multiplication> {
        let chars = self.data.chars().collect::<Vec<char>>();

        let mut multiplications = Vec::new();

        let mut add_mul = true;

        for (i, items) in chars.windows(4).enumerate() {
            // Look for initial patterns in the window.
            match (items[0], items[1], items[2], items[3]) {
                ('m', 'u', 'l', '(') => {}
                ('d', 'o', '(', ')') => {
                    add_mul = true;
                    continue;
                }
                ('d', 'o', 'n', '\'') => {
                    if chars[i + 4] == 't' && chars[i + 5] == '(' && chars[i + 6] == ')' {
                        add_mul = false;
                    }
                    continue;
                }
                _ => continue,
            }

            // Try to complete the full pattern now
            let first_part_result = Self::search_closing_pattern(&chars, i + 3, ',');

            if first_part_result.is_none() {
                continue;
            }

            let (first_number, next_index) = first_part_result.unwrap();

            let second_part_result = Self::search_closing_pattern(&chars, next_index, ')');

            if second_part_result.is_none() {
                continue;
            }

            let (second_number, _) = second_part_result.unwrap();

            // Only enable do/don't when skipping
            if !skip {
                add_mul = true;
            }

            if add_mul {
                multiplications.push(Multiplication(first_number, second_number));
            }
        }

        multiplications
    }

    // Not my biggest achivement in life
    fn search_closing_pattern(
        items: &[char],
        initial_index: usize,
        closing_delimeter: char,
    ) -> Option<(u32, usize)> {
        let result_number;
        let ending_index;

        if items[initial_index + 1].is_numeric() {
            if items[initial_index + 2] == closing_delimeter {
                result_number = items[initial_index + 1]
                    .to_digit(10)
                    .expect("Can't parse char");
                ending_index = initial_index + 2;
            } else if items[initial_index + 2].is_numeric()
                && items[initial_index + 3] == closing_delimeter
            {
                result_number = items[initial_index + 1]
                    .to_digit(10)
                    .expect("Can't parse char")
                    * 10
                    + items[initial_index + 2]
                        .to_digit(10)
                        .expect("Can't parse char");
                ending_index = initial_index + 3;
            } else if items[initial_index + 3].is_numeric()
                && items[initial_index + 4] == closing_delimeter
            {
                result_number = items[initial_index + 1]
                    .to_digit(10)
                    .expect("Can't parse char")
                    * 100
                    + items[initial_index + 2]
                        .to_digit(10)
                        .expect("Can't parse char")
                        * 10
                    + items[initial_index + 3]
                        .to_digit(10)
                        .expect("Can't parse char");
                ending_index = initial_index + 4;
            } else {
                return None;
            }
        } else {
            return None;
        }

        Some((result_number, ending_index))
    }

    fn get_valid_simple_multiplications_result(&self) -> u32 {
        self.get_valid_multiplications(false)
            .iter()
            .map(|x| x.0 * x.1)
            .sum()
    }

    fn get_valid_extra_multiplications_result(&self) -> u32 {
        self.get_valid_multiplications(true)
            .iter()
            .map(|x| x.0 * x.1)
            .sum()
    }
}

struct Multiplication(u32, u32);

fn main() {
    let memory_data = Memory::parse("day3/data/input.txt");

    let simple_multiplications_result = memory_data.get_valid_simple_multiplications_result();

    println!("Part 1 result: {}", simple_multiplications_result);

    let extra_multiplications_result = memory_data.get_valid_extra_multiplications_result();

    println!("Part 2 result: {}", extra_multiplications_result);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> Memory {
        Memory {
            data: String::from(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            ),
        }
    }

    #[test]
    fn check_parsing() {
        let input = Memory::parse("data/test1.txt");

        let test_data = get_test_input();

        assert_eq!(input.data, test_data.data);
    }

    #[test]
    fn check_good_multiplications_count() {
        let test_data = get_test_input();

        assert_eq!(test_data.get_valid_multiplications(false).len(), 4);
    }

    #[test]
    fn check_simple_multiplications_result() {
        let test_data = get_test_input();

        assert_eq!(test_data.get_valid_simple_multiplications_result(), 161);
    }

    #[test]
    fn check_extra_multiplications_result() {
        let test_data = Memory::parse("data/test2.txt");

        assert_eq!(test_data.get_valid_extra_multiplications_result(), 48);
    }
}
