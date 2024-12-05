use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq)]
struct PageOrder {
    page: u32,
    before: u32,
}

type Page = u32;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Update {
    data: Vec<Page>,
}

#[derive(Debug, PartialEq, Eq)]
struct InputData {
    ordering_rules: Vec<PageOrder>,
    updates: Vec<Update>,
}

impl InputData {
    fn parse(file_path: &str) -> Self {
        let file = File::open(file_path).expect("File can't be read");
        let reader = BufReader::new(file);

        let mut rules = Vec::new();
        let mut updates = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();

            if line.is_empty() {
                continue;
            }

            if line.contains('|') {
                let items: Vec<u32> = line
                    .split('|')
                    .map(|x| x.parse().expect("Can't parse number"))
                    .collect();

                rules.push(PageOrder {
                    page: items[0],
                    before: items[1],
                });
            } else if line.contains(',') {
                let items: Vec<u32> = line
                    .split(',')
                    .map(|x| x.parse().expect("Can't parse number"))
                    .collect();

                updates.push(Update { data: items });
            }
        }

        Self {
            ordering_rules: rules,
            updates,
        }
    }
}

struct UpdatesChecker {
    update_rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Update>,
}

struct CategorizedUpdates {
    good_updates: Vec<Update>,
    bad_updates: Vec<Update>,
}

impl UpdatesChecker {
    fn new(data: InputData) -> Self {
        let mut update_rules: HashMap<u32, Vec<u32>> = HashMap::new();

        data.ordering_rules.iter().for_each(|rule| {
            update_rules
                .entry(rule.page)
                .and_modify(|value| value.push(rule.before))
                .or_insert(vec![rule.before]);
        });

        Self {
            update_rules,
            updates: data.updates,
        }
    }

    fn check_updates(&self) -> CategorizedUpdates {
        let mut good_updates: Vec<Update> = Vec::new();
        let mut bad_updates: Vec<Update> = Vec::new();

        for update in self.updates.iter() {
            if self.is_update_good(update) {
                good_updates.push(update.clone());
            } else {
                bad_updates.push(update.clone());
            }
        }

        CategorizedUpdates {
            good_updates,
            bad_updates,
        }
    }

    fn is_update_good(&self, update: &Update) -> bool {
        for (i, page) in update.data.iter().enumerate() {
            let page_rules = self.update_rules.get(page);

            // No rules, then we are fine!
            if page_rules.is_none() {
                continue;
            }

            let page_rules = page_rules.unwrap();

            // Check that none of the pages appear before that one
            for check_page in page_rules {
                if update.data[0..i].contains(check_page) {
                    return false;
                }
            }
        }

        true
    }

    fn fix_updates(&self, updates: &mut [Update]) {
        updates
            .iter_mut()
            .for_each(|update| self.fix_update(update));
    }

    fn fix_update(&self, update: &mut Update) {
        loop {
            let mut swap: Option<(usize, usize)> = None;

            'page: for (i, page) in update.data.iter().enumerate() {
                let page_rules = self.update_rules.get(page);

                // No rules, then we are fine!
                if page_rules.is_none() {
                    continue;
                }

                let page_rules = page_rules.unwrap();

                // Find wrong order
                for check_page in page_rules.iter() {
                    for (j, previous_page) in update.data[0..i].iter().enumerate() {
                        if previous_page == check_page {
                            swap = Some((i, j));
                            break 'page;
                        }
                    }
                }
            }

            // If an error was found, try to fix it and keep checking
            if let Some((i, j)) = swap {
                update.data.swap(i, j);
                continue;
            }

            // Otherwise, the update is finally valid
            break;
        }
    }

    fn get_updates_result(updates: &[Update]) -> u32 {
        updates
            .iter()
            .map(|update| {
                let index_middle = update.data.len().div_euclid(2);
                update.data[index_middle]
            })
            .sum()
    }
}

fn main() {
    let input_data = InputData::parse("day5/data/input.txt");

    let updates_checker = UpdatesChecker::new(input_data);

    let mut checked_updates = updates_checker.check_updates();

    let good_updates_result = UpdatesChecker::get_updates_result(&checked_updates.good_updates);

    println!("Part 1 result: {}", good_updates_result);

    updates_checker.fix_updates(&mut checked_updates.bad_updates);

    let fixed_updates_result = UpdatesChecker::get_updates_result(&checked_updates.bad_updates);

    println!("Part 2 result: {}", fixed_updates_result);
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> InputData {
        InputData {
            ordering_rules: vec![
                PageOrder {
                    page: 47,
                    before: 53,
                },
                PageOrder {
                    page: 97,
                    before: 13,
                },
                PageOrder {
                    page: 97,
                    before: 61,
                },
                PageOrder {
                    page: 97,
                    before: 47,
                },
                PageOrder {
                    page: 75,
                    before: 29,
                },
                PageOrder {
                    page: 61,
                    before: 13,
                },
                PageOrder {
                    page: 75,
                    before: 53,
                },
                PageOrder {
                    page: 29,
                    before: 13,
                },
                PageOrder {
                    page: 97,
                    before: 29,
                },
                PageOrder {
                    page: 53,
                    before: 29,
                },
                PageOrder {
                    page: 61,
                    before: 53,
                },
                PageOrder {
                    page: 97,
                    before: 53,
                },
                PageOrder {
                    page: 61,
                    before: 29,
                },
                PageOrder {
                    page: 47,
                    before: 13,
                },
                PageOrder {
                    page: 75,
                    before: 47,
                },
                PageOrder {
                    page: 97,
                    before: 75,
                },
                PageOrder {
                    page: 47,
                    before: 61,
                },
                PageOrder {
                    page: 75,
                    before: 61,
                },
                PageOrder {
                    page: 47,
                    before: 29,
                },
                PageOrder {
                    page: 75,
                    before: 13,
                },
                PageOrder {
                    page: 53,
                    before: 13,
                },
            ],
            updates: vec![
                Update {
                    data: vec![75, 47, 61, 53, 29],
                },
                Update {
                    data: vec![97, 61, 53, 29, 13],
                },
                Update {
                    data: vec![75, 29, 13],
                },
                Update {
                    data: vec![75, 97, 47, 61, 53],
                },
                Update {
                    data: vec![61, 13, 29],
                },
                Update {
                    data: vec![97, 13, 75, 29, 47],
                },
            ],
        }
    }

    #[test]
    fn check_parsing() {
        let input = InputData::parse("data/test.txt");

        let test_data = get_test_input();

        assert_eq!(input, test_data);
    }

    #[test]
    fn check_correct_updates_result() {
        let input_data = get_test_input();

        let updates_checker = UpdatesChecker::new(input_data);
        let checked_updates = updates_checker.check_updates();

        let good_updates_result = UpdatesChecker::get_updates_result(&checked_updates.good_updates);

        assert_eq!(good_updates_result, 143);
    }

    #[test]
    fn check_fix_update() {
        let input_data = get_test_input();

        let updates_checker = UpdatesChecker::new(input_data);

        let mut bad_update = Update {
            data: vec![75, 97, 47, 61, 53],
        };

        updates_checker.fix_update(&mut bad_update);

        assert_eq!(
            bad_update,
            Update {
                data: vec![97, 75, 47, 61, 53]
            }
        );
    }

    #[test]
    fn check_fixed_updates_result() {
        let input_data = get_test_input();

        let updates_checker = UpdatesChecker::new(input_data);
        let mut checked_updates = updates_checker.check_updates();

        updates_checker.fix_updates(&mut checked_updates.bad_updates);

        let updates_result = UpdatesChecker::get_updates_result(&checked_updates.bad_updates);

        assert_eq!(updates_result, 123);
    }
}
