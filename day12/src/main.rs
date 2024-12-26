use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use common::{Direction, Position};

struct Region {
    #[allow(dead_code)]
    id: char,
    items: HashSet<Position>,
}

impl Region {
    fn get_area(&self) -> usize {
        self.items.len()
    }

    fn get_perimeter(&self) -> usize {
        let mut perimeter = 0;

        let directions = Direction::generate_basic_directions_list();

        for item in self.items.iter() {
            for direction in directions.iter() {
                let new_pos = Direction::apply_offset(direction, item.x, item.y);

                if !self.items.contains(&new_pos) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    fn get_sides(&self) -> usize {
        let mut sides = 0;

        let directions = Direction::generate_basic_directions_list();
        let mut items_wall: [HashMap<isize, Vec<isize>>; 4] = [
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        ];

        // Classify walls
        for item in self.items.iter() {
            for direction in directions.iter() {
                let new_pos = Direction::apply_offset(direction, item.x, item.y);

                if self.items.contains(&new_pos) {
                    continue;
                }

                match direction {
                    Direction::Top | Direction::Bottom => items_wall[direction.clone() as usize]
                        .entry(new_pos.y)
                        .and_modify(|items| items.push(new_pos.x))
                        .or_insert(vec![new_pos.x]),
                    Direction::Right | Direction::Left => items_wall[direction.clone() as usize]
                        .entry(new_pos.x)
                        .and_modify(|items| items.push(new_pos.y))
                        .or_insert(vec![new_pos.y]),
                    Direction::TopLeft => unreachable!(),
                    Direction::TopRight => unreachable!(),
                    Direction::BottomRight => unreachable!(),
                    Direction::BottomLeft => unreachable!(),
                };
            }
        }

        // Deduplicate walls
        for items_wall in items_wall.iter_mut() {
            for (_, items) in items_wall.iter_mut() {
                // A line of walls is split by gaps between them, thus items must be sorted first
                items.sort();

                let mut sides_counter = 1;

                for pair in items.windows(2) {
                    if pair[0].abs_diff(pair[1]) > 1 {
                        sides_counter += 1;
                    }
                }

                sides += sides_counter;
            }
        }

        sides
    }

    fn calculate_price(&self) -> u64 {
        self.get_area() as u64 * self.get_perimeter() as u64
    }

    fn calculate_bulk_price(&self) -> u64 {
        self.get_area() as u64 * self.get_sides() as u64
    }
}

struct Garden(Vec<Region>);

impl Garden {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let mut garden_data = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();

            let line_data: Vec<char> = line.chars().collect();

            garden_data.push(line_data);
        }

        let regions = Self::get_regions(&garden_data);

        Self(regions)
    }

    fn get_regions(data: &[Vec<char>]) -> Vec<Region> {
        let mut regions = Vec::new();
        let mut visited_pos = HashSet::new();

        for (y, row) in data.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let position = Position {
                    x: x as isize,
                    y: y as isize,
                };

                if visited_pos.contains(&position) {
                    continue;
                }

                let region = Self::get_region(data, position);

                visited_pos.extend(region.items.clone());

                regions.push(region);
            }
        }

        regions
    }

    fn get_region(data: &[Vec<char>], initial_pos: Position) -> Region {
        let region_id = data[initial_pos.y as usize][initial_pos.x as usize];
        let mut region_items = HashSet::new();

        let directions = Direction::generate_basic_directions_list();

        let mut next_positions = vec![initial_pos];

        while let Some(next_pos) = next_positions.pop() {
            if !Self::is_within_bounds(data, &next_pos) {
                continue;
            }

            let item = data[next_pos.y as usize][next_pos.x as usize];

            if item != region_id {
                continue;
            }

            region_items.insert(next_pos.clone());

            for direction in directions.iter() {
                let new_pos = Direction::apply_offset(direction, next_pos.x, next_pos.y);

                if region_items.contains(&new_pos) {
                    continue;
                }

                next_positions.push(new_pos);
            }
        }

        Region {
            id: region_id,
            items: region_items,
        }
    }

    fn is_within_bounds(data: &[Vec<char>], pos: &Position) -> bool {
        pos.x >= 0 && pos.x < data[0].len() as isize && pos.y >= 0 && pos.y < data.len() as isize
    }

    fn calculate_total_price(&self) -> u64 {
        self.0.iter().map(|x| x.calculate_price()).sum()
    }

    fn calculate_total_bulk_price(&self) -> u64 {
        self.0.iter().map(|x| x.calculate_bulk_price()).sum()
    }
}

fn main() {
    let garden = Garden::parse("day12/data/input.txt");

    println!("Part 1 result: {}", garden.calculate_total_price());

    println!("Part 2 result: {}", garden.calculate_total_bulk_price());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_regions_test1() {
        let input = Garden::parse("data/test1.txt");

        assert_eq!(input.0.len(), 5);

        for region in input.0.iter() {
            match region.id {
                'A' => {
                    assert_eq!(region.get_area(), 4);
                    assert_eq!(region.get_perimeter(), 10);
                }
                'B' => {
                    assert_eq!(region.get_area(), 4);
                    assert_eq!(region.get_perimeter(), 8);
                }
                'C' => {
                    assert_eq!(region.get_area(), 4);
                    assert_eq!(region.get_perimeter(), 10);
                }
                'D' => {
                    assert_eq!(region.get_area(), 1);
                    assert_eq!(region.get_perimeter(), 4);
                }
                'E' => {
                    assert_eq!(region.get_area(), 3);
                    assert_eq!(region.get_perimeter(), 8);
                }
                _ => unreachable!("Invalid ID"),
            }
        }
    }

    #[test]
    fn check_regions_test2() {
        let input = Garden::parse("data/test2.txt");

        assert_eq!(input.0.len(), 5);

        for region in input.0.iter() {
            match region.id {
                'O' => {
                    assert_eq!(region.get_area(), 21);
                    assert_eq!(region.get_perimeter(), 36);
                }
                'X' => {
                    assert_eq!(region.get_area(), 1);
                    assert_eq!(region.get_perimeter(), 4);
                }
                _ => unreachable!("Invalid ID"),
            }
        }
    }

    #[test]
    fn check_total_price_test3() {
        let input = Garden::parse("data/test3.txt");

        assert_eq!(input.calculate_total_price(), 1930);
    }

    #[test]
    fn check_total_bulk_price_test1() {
        let input = Garden::parse("data/test1.txt");

        assert_eq!(input.calculate_total_bulk_price(), 80);
    }

    #[test]
    fn check_total_bulk_price_test2() {
        let input = Garden::parse("data/test2.txt");

        assert_eq!(input.calculate_total_bulk_price(), 436);
    }

    #[test]
    fn check_total_bulk_price_test3() {
        let input = Garden::parse("data/test3.txt");

        assert_eq!(input.calculate_total_bulk_price(), 1206);
    }

    #[test]
    fn check_total_bulk_price_test4() {
        let input = Garden::parse("data/test4.txt");

        assert_eq!(input.calculate_total_bulk_price(), 236);
    }

    #[test]
    fn check_total_bulk_price_test5() {
        let input = Garden::parse("data/test5.txt");

        assert_eq!(input.calculate_total_bulk_price(), 368);
    }
}
