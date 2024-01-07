fn main() {
    let input: &str = include_str!("./day1.txt");
    let pt1 = part_1(input);
    println!("Part 1: {}", pt1);
}

fn part_1(input: &str) -> usize {
    let mut total: usize = 0;
    
    for line in input.lines()  {
        let nums: Vec<char> = line.chars()
            .filter(|char| char.is_ascii_digit())
            .collect();
        let first = nums.first().unwrap();
        let last = nums.last().unwrap();
        let num = format!("{}{}", first, last)
            .parse::<usize>()
            .unwrap();
        total = total + num;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use crate::part_1;

    #[test]
    fn test_part_1() {
        let input = 
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(part_1(input), 142);
    }
}