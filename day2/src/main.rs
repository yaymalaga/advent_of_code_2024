use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Reports {
    data: Vec<Report>,
}

impl Reports {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let mut reports_data = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            let report_data: Vec<u32> = line
                .split(" ")
                .map(|x| x.parse::<u32>().expect("Can't convert to digit"))
                .collect();

            reports_data.push(Report::from(report_data));
        }

        Self { data: reports_data }
    }

    fn count_safe_reports(&self) -> usize {
        self.data
            .iter()
            .map(|report| report.is_safe())
            .filter(|&is_safe| is_safe)
            .count()
    }

    fn count_safe_tolerate_reports(&self) -> usize {
        self.data
            .iter()
            .map(|report| report.is_safe_tolerate())
            .filter(|&is_safe| is_safe)
            .count()
    }
}

struct Report {
    data: Vec<u32>,
}

impl Report {
    fn from(data: Vec<u32>) -> Self {
        Self { data }
    }

    fn is_safe(&self) -> bool {
        Self::is_report_safe(&self.data)
    }

    fn is_safe_tolerate(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        // Try again removing one value at the time
        for i in 0..self.data.len() {
            let mut new_data = self.data.clone();

            new_data.remove(i);

            if Self::is_report_safe(&new_data) {
                return true;
            }
        }

        false
    }

    fn is_report_safe(data: &[u32]) -> bool {
        let mut global_ordering: Option<Ordering> = None;

        for window in data.windows(2) {
            let current = window[0];
            let next = window[1];

            // Always increasing or decreasing
            if current == next {
                return false;
            }

            let ordering = if current > next {
                Ordering::Decresing
            } else {
                Ordering::Incresing
            };

            match &global_ordering {
                Some(value) => {
                    // Globally it can just increase or decrease
                    if *value != ordering {
                        return false;
                    }
                }
                None => global_ordering = Some(ordering),
            }

            // Diff should be between 1 and 3
            if current.abs_diff(next) > 3 {
                return false;
            }
        }

        true
    }
}

#[derive(PartialEq)]
enum Ordering {
    Decresing,
    Incresing,
}

fn main() {
    let reports = Reports::parse("day2/data/input.txt");

    let safe_reports_count = reports.count_safe_reports();

    println!("Part 1 result: {}", safe_reports_count);

    let safe_tolerate_reports_count = reports.count_safe_tolerate_reports();

    println!("Part 2 result: {}", safe_tolerate_reports_count);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> Reports {
        Reports {
            data: vec![
                Report {
                    data: vec![7, 6, 4, 2, 1],
                },
                Report {
                    data: vec![1, 2, 7, 8, 9],
                },
                Report {
                    data: vec![9, 7, 6, 2, 1],
                },
                Report {
                    data: vec![1, 3, 2, 4, 5],
                },
                Report {
                    data: vec![8, 6, 4, 4, 1],
                },
                Report {
                    data: vec![1, 3, 6, 7, 9],
                },
            ],
        }
    }

    #[test]
    fn check_parsing() {
        let input = Reports::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input.data.len(), test_data.data.len());

        for i in 0..input.data.len() {
            assert_eq!(input.data[i].data, test_data.data[i].data);
        }
    }

    #[test]
    fn check_safe_reports() {
        let reports = get_test_input();

        let safe_reports_count = reports.count_safe_reports();

        assert_eq!(safe_reports_count, 2);
    }

    #[test]
    fn check_safe_tolerate_reports() {
        let reports = get_test_input();

        let safe_tolerate_reports_count = reports.count_safe_tolerate_reports();

        assert_eq!(safe_tolerate_reports_count, 4);
    }
}
