fn common_chars(first: &str, second: &str) -> String {
    first
        .chars()
        .filter(|c| second.contains(*c))
        .collect::<String>()
}

fn char_value(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - b'a' as usize + 1,
        'A'..='Z' => c as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}

/// Find the item type that appears in both compartments of each rucksack.
/// What is the sum of the priorities of those item types?
pub fn part_1(input: &str) -> String {
    let mut score: usize = 0;
    for line in input.trim().lines() {
        let first = line[..(line.len() / 2)].to_string();
        let second = line[(line.len() / 2)..].to_string();

        score += char_value(common_chars(&first, &second).chars().nth(0).unwrap());
    }

    score.to_string()
}

/// Find the item type that corresponds to the badges of each three-Elf group.
/// What is the sum of the priorities of those item types?
pub fn part_2(input: &str) -> String {
    let mut score = 0;
    for group in input.trim().lines().collect::<Vec<&str>>().chunks(3) {
        let common = common_chars(group[0], &common_chars(&group[1], &group[2]));
        score += char_value(common.chars().nth(0).unwrap());
    }

    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "157");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../inputs/3")), "7824");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_2(INPUT), "70");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../inputs/3")), "2798");
    }
}
