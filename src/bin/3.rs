use std::{collections::HashMap, ops::Add};

#[advent_23::aoc(3)]
fn main(input: &str) -> (usize, usize) {
    let mut coords_raw = Vec::<(Vec<char>, Vec<(usize, usize)>)>::new();
    let max_width = 140;

    let chars = input.chars();

    let mut x = 0;
    let mut y = 0;
    let mut is_new = true;
    chars.for_each(|c| {
        if c == '\n' {
            y += 1;
            x = 0;

            is_new = true;
            return;
        }
        if !c.is_numeric() {
            x += 1;

            is_new = true;
            return;
        }
        if is_new {
            coords_raw.push((vec![c], vec![(x, y)]));
            is_new = false;
        } else {
            let mut copy = coords_raw.last().expect("Cannot get last element").clone();
            copy.0.push(c);
            copy.1.push((x, y));
            coords_raw.pop();
            coords_raw.push(copy);
        }
        x += 1;
    });

    // (x, y) (value, id)
    let mut coordinates = HashMap::<(usize, usize), (usize, usize)>::new();
    let mut id = 0;
    for (nr_raw, coords) in coords_raw {
        id += 1;
        let number = nr_raw.iter().collect::<String>().parse::<usize>().unwrap();
        for coord in coords {
            coordinates.insert(coord, (number, id));
        }
    }

    let mut x = 0;
    let mut y = 0;
    let mut used_ids = Vec::<usize>::new();
    let mut part_nrs = Vec::<usize>::new();
    let mut gear_ratios = Vec::<usize>::new();
    for c in input.chars() {
        if c == '.' || c.is_numeric() {
            x += 1;
            continue;
        }
        if c == '\n' {
            y += 1;
            x = 0;
            continue;
        }
        let mut gear_nrs = Vec::<usize>::new();
        if x > 0 {
            if let Some((nr, id)) = coordinates.get(&(x - 1, y)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if y > 0 {
            if let Some((nr, id)) = coordinates.get(&(x, y - 1)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if x < (max_width - 1) {
            if let Some((nr, id)) = coordinates.get(&(x + 1, y)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if y < (max_width - 1) {
            if let Some((nr, id)) = coordinates.get(&(x, y + 1)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if y < (max_width - 1) && x < (max_width - 1) {
            if let Some((nr, id)) = coordinates.get(&(x + 1, y + 1)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if y > 0 && x > 0 {
            if let Some((nr, id)) = coordinates.get(&(x - 1, y - 1)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if y > 0 && x < (max_width - 1) {
            if let Some((nr, id)) = coordinates.get(&(x + 1, y - 1)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if x > 0 && y < (max_width - 1) {
            if let Some((nr, id)) = coordinates.get(&(x - 1, y + 1)) {
                if !used_ids.contains(id) {
                    used_ids.push(id.clone());
                    part_nrs.push(nr.clone());
                    gear_nrs.push(nr.clone());
                }
            }
        }
        if c == '*' && gear_nrs.len() == 2 {
            gear_ratios.push(
                gear_nrs
                    .iter()
                    .map(|x: &usize| x.clone())
                    .reduce(|a, b| a * b)
                    .unwrap(),
            );
        }
        x += 1;
    }

    let part_1: usize = part_nrs
        .iter()
        .map(|x: &usize| x.clone())
        .reduce(|a, b| a.add(b))
        .unwrap();

    let part_2: usize = gear_ratios
        .iter()
        .map(|x: &usize| x.clone())
        .reduce(|a, b| a.add(b))
        .unwrap();

    return (part_1, part_2);
}
