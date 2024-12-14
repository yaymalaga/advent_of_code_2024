use std::{collections::HashMap, fs::File, io::Read};

#[derive(Clone, Debug, PartialEq)]
struct StoneRecord {
    id: u64,
    count: usize,
}

impl StoneRecord {
    fn blink(&self) -> Vec<StoneRecord> {
        let mut new_stones = Vec::with_capacity(2);

        if self.id == 0 {
            new_stones.push(StoneRecord {
                id: 1,
                count: self.count,
            });
            return new_stones;
        }

        let stone_id_string = self.id.to_string();

        if stone_id_string.len() % 2 == 0 {
            let first_part = stone_id_string[0..stone_id_string.len() / 2].to_string();
            let second_part = stone_id_string[stone_id_string.len() / 2..].to_string();

            new_stones.push(StoneRecord {
                id: first_part.parse().expect("Cant' convert to number"),
                count: self.count,
            });
            new_stones.push(StoneRecord {
                id: second_part.parse().expect("Cant' convert to number"),
                count: self.count,
            });

            return new_stones;
        }

        new_stones.push(StoneRecord {
            id: self.id * 2024,
            count: self.count,
        });

        new_stones
    }
}

#[derive(Clone)]
struct StonesList(Vec<StoneRecord>);

impl StonesList {
    fn parse(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("File can't be read");

        let mut raw_data = String::new();
        file.read_to_string(&mut raw_data)
            .expect("Failed to dump into string");

        let data = raw_data
            .trim()
            .split(" ")
            .map(|item| StoneRecord {
                id: item.parse::<u64>().expect("Can't convert into digit"),
                count: 1,
            })
            .collect::<Vec<StoneRecord>>();

        Self(data)
    }

    fn blink(&mut self) {
        self.0 = self.0.iter().flat_map(|stone| stone.blink()).collect();
    }

    fn blink_n(&mut self, n_blinks: usize) {
        for _ in 0..n_blinks {
            self.blink();
            self.deduplicate();
        }
    }

    fn deduplicate(&mut self) {
        let mut map = HashMap::new();

        self.0.iter().for_each(|stone| {
            map.entry(stone.id)
                .and_modify(|x| *x += stone.count)
                .or_insert(stone.count);
        });

        self.0 = map
            .iter()
            .map(|(key, value)| StoneRecord {
                id: *key,
                count: *value,
            })
            .collect();
    }

    fn len(&self) -> usize {
        self.0.iter().fold(0, |i, stone| i + stone.count)
    }
}

fn main() {
    let mut stones_list = StonesList::parse("day11/data/input.txt");

    stones_list.blink_n(25);

    println!("Part 1 result: {}", stones_list.len());

    stones_list.blink_n(75 - 25);

    println!("Part 2 result: {}", stones_list.len());
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> StonesList {
        StonesList(vec![
            StoneRecord { id: 125, count: 1 },
            StoneRecord { id: 17, count: 1 },
        ])
    }

    #[test]
    fn check_parsing() {
        let input = StonesList::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input.0, test_data.0);
    }

    #[test]
    fn check_blink_n_size() {
        let mut test_data = get_test_input();

        test_data.blink_n(6);

        assert_eq!(test_data.len(), 22);

        test_data.blink_n(25 - 6);

        assert_eq!(test_data.len(), 55312);
    }
}
