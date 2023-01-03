fn parse_input(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (first, second) = line.split_once(',')?;
            let (a, b) = first.split_once('-')?;
            let (c, d) = second.split_once('-')?;

            Some((
                a.parse().ok()?,
                b.parse().ok()?,
                c.parse().ok()?,
                d.parse().ok()?,
            ))
        })
        .collect::<Vec<(u32, u32, u32, u32)>>()
}

/// In how many assignment pairs does one range fully contain the other?
pub fn part_1(input: &str) -> String {
    parse_input(input)
        .iter()
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count()
        .to_string()
}

/// In how many assignment pairs do the ranges overlap?
pub fn part_2(input: &str) -> String {
    parse_input(input)
        .iter()
        .filter(|(a, b, c, d)| !(b < c || a > d))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "2");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../inputs/4")), "462");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(part_2(INPUT), "4");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../inputs/4")), "835");
    }
}
