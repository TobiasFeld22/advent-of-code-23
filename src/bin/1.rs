#[advent_23::aoc(1)]
fn main(input: &str) -> (usize, usize) {
    let part1: usize = input
        .lines()
        .map(|line| parse_file(line, false))
        .reduce(|a, b| a + b)
        .unwrap();
    let part2: usize = input
        .lines()
        .map(|line| parse_file(line, true))
        .reduce(|a, b| a + b)
        .unwrap();

    return (part1, part2);
}

fn parse_file(input: &str, parse_text: bool) -> usize {
    let mut input: String = format!("{}", input);
    if parse_text {
        input = input
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
    }
    let numbers = input.chars().filter(|c| c.is_numeric()).collect::<String>();
    let new_numbers = if numbers.len() == 1 {
        numbers.repeat(2)
    } else {
        format!(
            "{}{}",
            numbers.chars().next().unwrap(),
            numbers.chars().next_back().unwrap()
        )
    };

    new_numbers.parse::<usize>().expect("cannot parse numbers")
}
