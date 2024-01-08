struct Game {
    id: usize,
    sets: Vec<SetCubes>,
}

struct SetCubes {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let input: &str = include_str!("./day2.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {}", pt1);
    // println!("Part 2: {}", pt2);
}

fn part_1(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(input), 8);
    }

    // use crate::part_2;
    // #[test]
    // fn test_part_2() {
    //     let input = "two1nine
    //         eightwothree
    //         abcone2threexyz
    //         xtwone3four
    //         4nineeightseven2
    //         zoneight234
    //         7pqrstsixteen";
    //     assert_eq!(part_2(input), 281);
    // }
}
