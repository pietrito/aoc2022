/// What would your total score be if everything goes exactly according to your strategy guide?
pub fn part_1(input: &str) -> String {
    let mut score = 0;
    for line in input.trim().lines() {
        let opponent = line.chars()[0];
        let me = line.chars()[2];
    }
    todo!()
}

pub fn part_2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "".to_string());
    }

    /*
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../inputs/")), "".to_string());
    }
    */

    /*
    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "".to_string());
    }

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "".to_string());
    }
    */
}
