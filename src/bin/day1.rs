fn main() {
    let input: &str = include_str!("./day1.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
}

fn part_1(input: &str) -> usize {
    return input
        .lines()
        .fold(0, |sum, line| sum + calibration_value(line));
}

fn part_2(input: &str) -> usize {
    return input
        .lines()
        .fold(0, |sum, line| sum + calibration_value(&extract_nums(line)));
}

fn calibration_value(line: &str) -> usize {
    let nums: Vec<char> = line.chars().filter(|char| char.is_ascii_digit()).collect();
    let first = nums.first().unwrap();
    let last = nums.last().unwrap();
    return format!("{}{}", first, last).parse::<usize>().unwrap();
}

fn word_to_num(input: &str) -> Option<char> {
    match input {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn extract_nums(line: &str) -> String {
    let mut str: String = "".to_string();
    for (left, char) in line.char_indices() {
        for right in left..line.len() {
            let num = word_to_num(&line[left..=right]);
            if num.is_some() {
                str.push(num.unwrap());
            }
        }
        str.push(char)
    }
    return str;
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        assert_eq!(part_1(input), 142);
    }

    use crate::part_2;
    #[test]
    fn test_part_2() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        assert_eq!(part_2(input), 281);
    }
}
