use itertools::Itertools;

#[advent_23::aoc(2)]
fn main(input: &str) -> (usize, usize) {
    let games = parse_game(input);
    let part_1_sum = games
        .iter()
        .filter(|game| game.max.red <= 12 && game.max.green <= 13 && game.max.blue <= 14)
        .map(|game| game.id)
        .reduce(|a, b| a + b)
        .unwrap();

    let part_2_sum = games
        .iter()
        .map(|game| game.max.red * game.max.blue * game.max.green)
        .reduce(|a, b| a + b)
        .unwrap();

    return (part_1_sum, part_2_sum);
}

fn parse_game(input: &str) -> Vec<Game> {
    return input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut split = line.split(" ");
            split.next();
            split.next();
            (id + 1, split.join(" "))
        })
        .map(|(id, x)| Game::new(id, parse_rounds(&x)))
        .collect_vec();
}

fn parse_rounds(input: &str) -> Vec<Round> {
    let rounds_raw = input.split(";");
    let parsed_rounds = rounds_raw.map(Round::new).collect_vec();

    return parsed_rounds;
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
    max: Round,
}

impl Game {
    fn new(id: usize, rounds: Vec<Round>) -> Self {
        let mut max = Round {
            blue: 0,
            red: 0,
            green: 0,
        };
        rounds.iter().for_each(|round| {
            if max.blue < round.blue {
                max.blue = round.blue;
            }
            if max.green < round.green {
                max.green = round.green;
            }
            if max.red < round.red {
                max.red = round.red;
            }
        });
        Self { id, rounds, max }
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    blue: usize,
    red: usize,
    green: usize,
}

impl Round {
    fn new(input: &str) -> Self {
        let mut initial = Self {
            blue: 0,
            red: 0,
            green: 0,
        };

        input.split(", ").for_each(|chunk| {
            let number_str = chunk.trim().split(" ").next().unwrap();
            let number = number_str.parse::<usize>().unwrap();
            if chunk.contains("blue") {
                initial.blue += number;
            } else if chunk.contains("green") {
                initial.green += number;
            } else if chunk.contains("red") {
                initial.red += number;
            }
        });
        initial
    }
}
