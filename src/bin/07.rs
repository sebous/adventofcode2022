use std::collections::HashMap;

fn build_tree(input: &str) -> HashMap<String, u64> {
    let mut tree: HashMap<String, u64> = HashMap::new();
    let mut pwd: Vec<String> = vec![];

    for line in input.lines() {
        match &line[..4] {
            "$ cd" => match &line[5..] {
                ".." => {
                    pwd.pop();
                }
                x => {
                    pwd.push(x.to_owned() + if x == "/" { "" } else { "/" });
                    tree.insert(pwd.join(""), 0);
                }
            },
            "$ ls" | "dir " => continue,
            _ => {
                let (first, _) = line.split_once(" ").unwrap();
                let num: u64 = first.parse().unwrap();
                let current_path = pwd.join("");
                tree.iter_mut()
                    .filter(|(path, _)| current_path.starts_with(*path))
                    .for_each(|(_, value)| *value += num);
            }
        }
    }
    tree
}

pub fn part_one(input: &str) -> Option<u64> {
    let tree = build_tree(input);
    let total_size = tree.values().filter(|val| *val <= &(100000 as u64)).sum();
    Some(total_size)
}

pub fn part_two(input: &str) -> Option<u64> {
    let tree = build_tree(input);
    let total_space: u64 = 70000000;
    let needed_space: u64 = 30000000;
    let used_space = tree.get("/").unwrap();
    tree.values()
        .filter(|val| (used_space - *val) <= total_space - needed_space)
        .min()
        .map(|val| val.to_owned())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
