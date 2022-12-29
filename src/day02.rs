/// A = X = Rock
/// B = Y = Paper
/// C = Z = Scissors

fn round_score(other: char, me: char) -> u32 {
    match (other, me) {
        ('A', 'X') => 3 + 1,
        ('A', 'Y') => 6 + 2,
        ('A', 'Z') => 0 + 3,
        ('B', 'X') => 0 + 1,
        ('B', 'Y') => 3 + 2,
        ('B', 'Z') => 6 + 3,
        ('C', 'X') => 6 + 1,
        ('C', 'Y') => 0 + 2,
        ('C', 'Z') => 3 + 3,
        _ => unreachable!(),
    }
}

/// What would your total score be if everything goes exactly according to your strategy guide?
pub fn part_1(input: &str) -> String {
    let mut score = 0;
    for line in input.trim().lines() {
        let opponent = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();

        score += round_score(opponent, me);
    }

    score.to_string()
}

pub fn part_2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_example_1() {
        assert_eq!(part_1(INPUT), "15");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../inputs/2")), "10994");
    }

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
