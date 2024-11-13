use regex::Regex;

#[derive(Debug)]
struct Card {
    id: String,
    winning_numbers: Vec<u8>,
    numbers_you_have: Vec<u8>,
}

fn main() {
    let input: &str = include_str!("./day4.txt");
    let pt1 = part_1(input);
}

fn part_1(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().map(|line| parse_card(line)).collect();
    dbg!(cards);
    todo!();
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
    let id = groups["id"].to_string();
    let winning_numbers = to_u8_vec(&groups["winning_numbers"].to_string());
    let numbers_you_have = to_u8_vec(&groups["numbers_you_have"].to_string());

    Card {
        id,
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
