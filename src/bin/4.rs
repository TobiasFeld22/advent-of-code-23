use std::collections::HashMap;

use itertools::Itertools;

#[advent_23::aoc(4)]
fn main(input: &str) -> (usize, usize) {
    let mut part_1 = 0;
    let mut total_cards = HashMap::<usize, usize>::new();

    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let id = id + 1;
            let mut points = 0;
            let mut victories = 0;
            let value = line.split(": ").skip(1).next().unwrap();
            let mut split = value.split("|");
            let winners = get_numbers(split.next().unwrap());
            let options = get_numbers(split.next().unwrap().trim());
            winners.iter().for_each(|nr| {
                let contains: bool = options.contains(nr);
                if contains && points == 0 {
                    points = 1
                } else if contains {
                    points *= 2
                }
                if contains {
                    victories += 1;
                }
            });

            if let None = total_cards.get(&id) {
                total_cards.insert(id, 1);
            }
            let amount = total_cards.get(&id).unwrap().clone();

            for _ in 1..=amount {
                for nr in (id + 1)..=(id + victories) {
                    let current = total_cards.get(&nr).unwrap_or(&1);
                    total_cards.insert(nr, current + 1);
                }
            }

            part_1 += points;
        })
        .collect_vec();

    let part_2 = total_cards
        .values()
        .map(usize::clone)
        .reduce(|a, b| a + b)
        .unwrap();

    return (part_1, part_2);
}

fn get_numbers(input: &str) -> Vec<usize> {
    input
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}
