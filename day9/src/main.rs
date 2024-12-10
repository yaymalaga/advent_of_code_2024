use std::{fs::File, io::Read};

struct DiskMap {
    data: Vec<u32>,
}

#[derive(Clone, Debug)]
struct DiskBlocks {
    data: Vec<DiskBlock>,
}

#[derive(Clone, PartialEq, Debug)]
enum DiskBlock {
    File { id: u32 },
    Empty,
}

impl From<DiskMap> for DiskBlocks {
    fn from(disk_map: DiskMap) -> Self {
        let mut blocks_data = Vec::new();

        let mut file_index = 0;

        for (i, &item) in disk_map.data.iter().enumerate() {
            // Par numbers are file blocks, otherwise empty
            let block_type = if i % 2 == 0 {
                let file = DiskBlock::File { id: file_index };

                file_index += 1;

                file
            } else {
                DiskBlock::Empty
            };

            // As block as many times as the diskmap's item number
            for _ in 0..item {
                blocks_data.push(block_type.clone());
            }
        }

        Self { data: blocks_data }
    }
}

impl From<DiskBlocks> for String {
    fn from(disk_blocks: DiskBlocks) -> Self {
        let mut blocks_data = String::new();

        disk_blocks.data.iter().for_each(|item| {
            let letter: &str = match item {
                DiskBlock::File { id } => &format!("{}", id),
                DiskBlock::Empty => ".",
            };

            blocks_data.push_str(letter);
        });

        blocks_data
    }
}

impl DiskMap {
    fn parse(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("File can't be read");

        let mut raw_data = String::new();
        file.read_to_string(&mut raw_data)
            .expect("Failed to dump into string");

        let data = raw_data
            .trim()
            .chars()
            .map(|char| char.to_digit(10).expect("Can't convert into digit"))
            .collect::<Vec<u32>>();

        Self { data }
    }
}

impl DiskBlocks {
    fn optimize_by_block(&mut self) {
        // Move blocks from the right to the empty spaces on the left

        let mut left_index = 0;
        let mut right_index = self.data.len() - 1;

        loop {
            // Index from both sides until finding each pointers
            if left_index == right_index {
                break;
            }

            // Check we have a file on the right index
            let item_right = &self.data[right_index];

            if item_right == &DiskBlock::Empty {
                right_index -= 1;
                continue;
            }

            // Check we have an empty spot on the left index
            let item_left = &self.data[left_index];

            if item_left != &DiskBlock::Empty {
                left_index += 1;
                continue;
            }

            // We have a match, perform transfer
            let item_right = item_right.clone();
            let item_left = item_left.clone();

            self.data[right_index] = item_left;
            self.data[left_index] = item_right;
        }
    }

    fn optimize_by_file(&mut self) {
        // Move files from the right to the empty spaces on the left
        let mut right_index = self.data.len() - 1;

        loop {
            // Once right index reaches the left side, no more files to scan
            if right_index == 0 {
                break;
            }

            // Check if we have a file on the right index
            let item_right = &self.data[right_index];

            if item_right == &DiskBlock::Empty {
                right_index -= 1;
                continue;
            }

            // Check if the same item is repeated one after another (file)
            let mut item_right_size = 1;
            while &self.data[right_index - item_right_size] == item_right {
                item_right_size += 1;

                // Stop if we are reaching negative offset
                if item_right_size > right_index {
                    break;
                }
            }

            // Check for empty spots from the left side up to the right index
            let mut left_index = 0;

            while left_index < right_index {
                let item_left = &self.data[left_index];

                if item_left != &DiskBlock::Empty {
                    left_index += 1;
                    continue;
                }

                // Check for empty spaces one after another
                let mut item_left_size = 1;
                while &self.data[left_index + item_left_size] == item_left {
                    item_left_size += 1;
                }

                // Check if space is enough. Otherwise, skip all the spaces and keep searching
                if item_left_size < item_right_size {
                    left_index += item_left_size;
                    continue;
                }

                // Match found, perform transfer
                for i in 0..item_right_size {
                    let item_left_tmp = self.data[left_index + i].clone();
                    let item_right_tmp = self.data[right_index - i].clone();

                    self.data[left_index + i] = item_right_tmp;
                    self.data[right_index - i] = item_left_tmp;
                }

                break;
            }

            // Try to move right index to the next file, otherwise no more to scan
            if item_right_size > right_index {
                break;
            }
            right_index -= item_right_size;
        }
    }

    fn get_checksum(&self) -> u64 {
        let mut checksum = 0;

        for (i, item) in self.data.iter().enumerate() {
            if let DiskBlock::File { id } = item {
                checksum += *id as u64 * i as u64;
            }
        }

        checksum
    }
}

fn main() {
    let disk_map = DiskMap::parse("day9/data/input.txt");

    let mut disk_blocks: DiskBlocks = disk_map.into();

    let mut optimized_disk_blockes = disk_blocks.clone();
    optimized_disk_blockes.optimize_by_block();

    println!("Part 1 result: {}", optimized_disk_blockes.get_checksum());

    disk_blocks.optimize_by_file();

    println!("Part 2 result: {}", disk_blocks.get_checksum());
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> DiskMap {
        DiskMap {
            data: vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2],
        }
    }

    #[test]
    fn check_parsing() {
        let input = DiskMap::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input.data, test_data.data);
    }

    #[test]
    fn check_disk_map_to_disk_blocks() {
        let input = get_test_input();

        let disk_blocks: DiskBlocks = input.into();

        assert_eq!(
            String::from(disk_blocks),
            String::from("00...111...2...333.44.5555.6666.777.888899")
        );
    }

    #[test]
    fn check_disk_block_optimized_by_block_checksum() {
        let input = get_test_input();

        let mut disk_blocks: DiskBlocks = input.into();

        disk_blocks.optimize_by_block();

        assert_eq!(
            String::from(disk_blocks.clone()),
            String::from("0099811188827773336446555566..............")
        );

        let checksum = disk_blocks.get_checksum();

        assert_eq!(checksum, 1928);
    }

    #[test]
    fn check_disk_block_optimized_by_file_checksum() {
        let input = get_test_input();

        let mut disk_blocks: DiskBlocks = input.into();

        disk_blocks.optimize_by_file();

        assert_eq!(
            String::from(disk_blocks.clone()),
            String::from("00992111777.44.333....5555.6666.....8888..")
        );

        let checksum = disk_blocks.get_checksum();

        assert_eq!(checksum, 2858);
    }
}
