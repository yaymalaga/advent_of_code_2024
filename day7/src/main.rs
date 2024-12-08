use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat_n,
};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq)]
struct CalibrationEquation {
    result: u64,
    members: Vec<u64>,
}

#[derive(Debug, PartialEq)]
struct CalibrationEquations {
    data: Vec<CalibrationEquation>,
}

impl CalibrationEquations {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let mut equations = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();

            let result_members = line.split(": ").take(2).collect::<Vec<&str>>();

            let result = result_members[0]
                .parse::<u64>()
                .expect("Can't parse eq result to number");

            let members = result_members[1]
                .split(" ")
                .map(|x| x.parse::<u64>().expect("Can't parse eq member to number"))
                .collect::<Vec<u64>>();

            equations.push(CalibrationEquation { result, members });
        }

        Self { data: equations }
    }

    fn get_possible_equations(&self, skip_combination: bool) -> Self {
        let mut possible_equations = Vec::new();

        'equation: for equation in self.data.iter() {
            let operators_list = if skip_combination {
                Operator::get_basic_operators_list()
            } else {
                Operator::get_total_operators_list()
            };

            let operators_combinations =
                Operator::get_operators_combinations(operators_list, equation.members.len() - 1);

            for operators_combination in operators_combinations {
                let mut eq_numbers = VecDeque::from(equation.members.clone());
                let mut operators = operators_combination.clone();

                // Reverse operators to get right order when doing pop from the back
                operators.reverse();

                while eq_numbers.len() > 1 {
                    let first_number = eq_numbers.pop_front().expect("Number vec is empty");
                    let second_number = eq_numbers.pop_front().expect("Number vec is empty");

                    let operator = operators.pop().expect("Operator vec is empty");

                    let result = operator.resolve(first_number, second_number);

                    eq_numbers.push_front(result);
                }

                // Check result, if valid no more combinations are needed for it
                if eq_numbers[0] == equation.result {
                    possible_equations.push(equation.clone());
                    continue 'equation;
                }
            }
        }

        Self {
            data: possible_equations,
        }
    }

    fn get_equations_total_calibration_result(&self) -> u64 {
        self.data.iter().map(|x| x.result).sum()
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Sum,
    Multiplication,
    Combination,
}

impl Operator {
    fn get_basic_operators_list() -> Vec<Self> {
        Vec::from([Self::Sum, Self::Multiplication])
    }

    fn get_total_operators_list() -> Vec<Self> {
        Vec::from([Self::Sum, Self::Multiplication, Self::Combination])
    }

    fn get_operators_combinations(operators: Vec<Self>, size: usize) -> Vec<Vec<Operator>> {
        repeat_n(operators, size)
            .multi_cartesian_product()
            .collect()
    }

    fn resolve(&self, number_a: u64, number_b: u64) -> u64 {
        match &self {
            Operator::Sum => number_a + number_b,
            Operator::Multiplication => number_a * number_b,
            Operator::Combination => format!("{}{}", number_a, number_b).parse().unwrap(),
        }
    }
}

fn main() {
    let calibration_equations = CalibrationEquations::parse("day7/data/input.txt");

    let possible_equations = calibration_equations.get_possible_equations(true);

    let calibration_total_result = possible_equations.get_equations_total_calibration_result();

    println!("Part 1 result: {}", calibration_total_result);

    let possible_equations = calibration_equations.get_possible_equations(false);

    let calibration_total_result = possible_equations.get_equations_total_calibration_result();

    println!("Part 2 result: {}", calibration_total_result);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> CalibrationEquations {
        CalibrationEquations {
            data: vec![
                CalibrationEquation {
                    result: 190,
                    members: vec![10, 19],
                },
                CalibrationEquation {
                    result: 3267,
                    members: vec![81, 40, 27],
                },
                CalibrationEquation {
                    result: 83,
                    members: vec![17, 5],
                },
                CalibrationEquation {
                    result: 156,
                    members: vec![15, 6],
                },
                CalibrationEquation {
                    result: 7290,
                    members: vec![6, 8, 6, 15],
                },
                CalibrationEquation {
                    result: 161011,
                    members: vec![16, 10, 13],
                },
                CalibrationEquation {
                    result: 192,
                    members: vec![17, 8, 14],
                },
                CalibrationEquation {
                    result: 21037,
                    members: vec![9, 7, 18, 13],
                },
                CalibrationEquation {
                    result: 292,
                    members: vec![11, 6, 16, 20],
                },
            ],
        }
    }

    #[test]
    fn check_parsing() {
        let input = CalibrationEquations::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input, test_data);
    }

    #[test]
    fn check_possible_equations_count() {
        let test_data = get_test_input();

        let possible_equations = test_data.get_possible_equations(true);

        let calibration_total_result = possible_equations.get_equations_total_calibration_result();

        assert_eq!(calibration_total_result, 3749);
    }

    #[test]
    fn check_possible_equations_with_combinations_count() {
        let test_data = get_test_input();

        let possible_equations = test_data.get_possible_equations(false);

        let calibration_total_result = possible_equations.get_equations_total_calibration_result();

        assert_eq!(calibration_total_result, 11387);
    }
}
