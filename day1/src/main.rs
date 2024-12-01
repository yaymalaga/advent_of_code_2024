use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct InputData {
    list_a: Vec<u32>,
    list_b: Vec<u32>,
}

impl InputData {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let mut list_a = vec![];
        let mut list_b = vec![];

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let items: Vec<u32> = line
                .split("   ")
                .take(2)
                .map(|x| x.parse::<u32>().expect("Can't convert to digit"))
                .collect();

            list_a.push(items[0]);
            list_b.push(items[1]);
        }

        Self { list_a, list_b }
    }
}

struct ListsChecker {
    data: InputData,
}

impl ListsChecker {
    fn new(data: InputData) -> Self {
        Self { data }
    }

    fn get_total_distance(&mut self) -> u32 {
        self.data.list_a.sort();
        self.data.list_b.sort();

        self.data
            .list_a
            .iter()
            .zip(self.data.list_b.iter())
            .fold(0, |acc, (&item_a, &item_b)| acc + item_a.abs_diff(item_b))
    }

    fn get_similarity(&mut self) -> u32 {
        let mut items_b_count: HashMap<u32, u32> = HashMap::new();

        self.data.list_b.iter().for_each(|item| {
            items_b_count
                .entry(*item)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        });

        self.data
            .list_a
            .iter()
            .map(|x| x * items_b_count.get(x).unwrap_or(&0))
            .sum()
    }
}

fn main() {
    let input = InputData::parse("day1/data/input.txt");
    let mut lists_checker = ListsChecker::new(input);

    let total_distance = lists_checker.get_total_distance();

    println!("Part 1 result: {}", total_distance);

    let similarity = lists_checker.get_similarity();

    println!("Part 2 result: {}", similarity);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> InputData {
        InputData {
            list_a: vec![3, 4, 2, 1, 3, 3],
            list_b: vec![4, 3, 5, 3, 9, 3],
        }
    }

    #[test]
    fn check_parsing() {
        let input = InputData::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input.list_a, test_data.list_a);
        assert_eq!(input.list_b, test_data.list_b);
    }

    #[test]
    fn check_distance() {
        let test_data = get_test_input();

        let mut lists_checker = ListsChecker::new(test_data);

        let distance = lists_checker.get_total_distance();

        assert_eq!(distance, 11);
    }

    #[test]
    fn check_similarity() {
        let test_data = get_test_input();

        let mut lists_checker = ListsChecker::new(test_data);

        let similarity = lists_checker.get_similarity();

        assert_eq!(similarity, 31);
    }
}
