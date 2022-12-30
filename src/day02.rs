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

/// X = Lose
/// Y = Draw
/// Z = Win
fn map_move(other: char, me: char) -> (char, char) {
    match (other, me) {
        ('A', 'X') => ('A', 'Z'),
        ('A', 'Y') => ('A', 'X'),
        ('A', 'Z') => ('A', 'Y'),
        ('B', 'X') => ('B', 'X'),
        ('B', 'Y') => ('B', 'Y'),
        ('B', 'Z') => ('B', 'Z'),
        ('C', 'X') => ('C', 'Y'),
        ('C', 'Y') => ('C', 'Z'),
        ('C', 'Z') => ('C', 'X'),
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

/// Following the Elf's instructions for the second column, what would your total score be
/// if everything goes exactly according to your strategy guide?
pub fn part_2(input: &str) -> String {
    let mut score = 0;
    for line in input.trim().lines() {
        let opponent = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();
        let (opponent, me) = map_move(opponent, me);

        score += round_score(opponent, me);
    }

    score.to_string()
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

    #[test]
    fn test_example_2() {
        assert_eq!(part_2(INPUT), "12");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../inputs/2")), "12526");
    }
}
