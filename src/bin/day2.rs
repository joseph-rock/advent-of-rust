use regex::Regex;

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<SetCubes>,
}

#[derive(Debug)]
struct SetCubes {
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let input: &str = include_str!("./day2.txt");
    let pt1 = part_1(&input);
    let pt2 = part_2(&input);
    println!("Part 1: {}", pt1);
    println!("Part 2: {}", pt2);
}

fn part_1(input: &str) -> usize {
    let is_possible = |set: &SetCubes| set.red <= 12 && set.green <= 13 && set.blue <= 14;
    let possible_id = |game: &Game| {
        if game.sets.iter().all(|set| is_possible(set)) {
            game.id
        } else {
            0
        }
    };

    input
        .lines()
        .map(|line| parse_line(line))
        .map(|game| possible_id(&game))
        .fold(0, |sum, id| sum + id)
}

fn part_2(input: &str) -> usize {
    let games: Vec<Game> = input.lines().map(|line| parse_line(line)).collect();
    let mut total = 0;

    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in game.sets {
            if set.red > red {
                red = set.red.clone();
            }
            if set.green > green {
                green = set.green.clone();
            }
            if set.blue > blue {
                blue = set.blue.clone();
            }
        }
        total += red * green * blue
    }

    total
}

fn parse_line(line: &str) -> Game {
    let re = Regex::new(r"\w+\s(?<id>\d+)\:\s(?<sets>.+)").unwrap();
    let groups = re.captures(line).unwrap();

    let id: usize = groups["id"].parse().unwrap();
    let set_group: &str = &groups["sets"].to_string();
    let sets: Vec<SetCubes> = set_group.split(";").map(|set| parse_set(set)).collect();

    Game { id, sets }
}

fn parse_set(set_str: &str) -> SetCubes {
    let re = Regex::new(r"(?<amount>\d+)\s(?<color>.+)").unwrap();
    let set_vec: Vec<&str> = set_str.split(", ").collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube_str in set_vec {
        let bar = re.captures(cube_str).unwrap();
        let amount: usize = bar["amount"].to_string().parse().unwrap();
        let color: &str = &bar["color"].to_string();

        if color == "red" {
            red += amount;
        }
        if color == "green" {
            green += amount;
        }
        if color == "blue" {
            blue += amount;
        }
    }
    SetCubes { red, green, blue }
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn day_2_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(input), 8);
    }

    use crate::part_2;
    #[test]
    fn day_2_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_2(input), 2286);
    }
}
