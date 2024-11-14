use regex::Regex;

#[derive(Debug)]
struct Card {
    _id: String,
    winning_numbers: Vec<u8>,
    numbers_you_have: Vec<u8>,
}

fn main() {
    let input: &str = include_str!("./day4.txt");
    let pt1 = part_1(input);
    println!("Part 1: {}", pt1);
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| parse_card(line))
        .map(|card| matched_numbers(&card))
        .map(|matched_numbers| score(matched_numbers.len()))
        .fold(0, |sum, score| sum + score)
}

fn parse_card(line: &str) -> Card {
    let re = Regex::new(
        r"(?x) # multiline regex!
        (?<id>\w+\s+\d+)
        (\:\s+)
        (?<winning_numbers>\d.+)
        (\s+\|\s+)
        (?<numbers_you_have>\d.+)",
    )
    .unwrap();

    let groups = re.captures(line).unwrap();
    let _id = groups["id"].to_string();
    let winning_numbers = to_u8_vec(&groups["winning_numbers"].to_string());
    let numbers_you_have = to_u8_vec(&groups["numbers_you_have"].to_string());

    Card {
        _id,
        winning_numbers,
        numbers_you_have,
    }
}

fn to_u8_vec(input: &str) -> Vec<u8> {
    input
        .split_whitespace()
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
}

fn matched_numbers(card: &Card) -> Vec<u8> {
    card.numbers_you_have
        .clone()
        .into_iter()
        .filter(|num| card.winning_numbers.contains(num))
        .collect()
}

fn score(n: usize) -> usize {
    if n <= 0 {
        return 0;
    }
    let power = (n - 1) as u32;
    usize::pow(2, power)
}
