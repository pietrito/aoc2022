fn parse_groups(input: &str) -> Vec<usize> {
    input
        .trim()
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>()
}

pub fn part_1(input: &str) -> String {
    parse_groups(input).iter().max().unwrap().to_string()
}

pub fn part_2(input: &str) -> String {
    let mut groups = parse_groups(input);
    groups.sort();
    groups.iter().rev().take(3).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "24000".to_string());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../inputs/1")), "70764".to_string());
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_2(INPUT), "45000".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../inputs/1")), "203905".to_string());
    }
}
