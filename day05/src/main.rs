fn main() {
    part1();
    part2();
}

fn check_validity(rules: Vec<Vec<i32>>, update: Vec<i32>) -> Result<bool, (usize, usize)> {
    let update = update.clone();

    for rule in rules.clone() {
        if update.contains(&rule[0]) && update.contains(&rule[1]) {
            let pos1 = update.iter().position(|&x| x == rule[0]).unwrap();
            let pos2 = update.iter().position(|&x| x == rule[1]).unwrap();

            if pos1 > pos2 {
                return Err((pos1, pos2));
            }
        }
    }

    Ok(true)
}

fn part1() {
    let (rules, updates) = get_input();

    let valid_updates = updates
        .iter()
        .filter(|update| check_validity(rules.clone(), update.clone().to_owned()).is_ok())
        .cloned()
        .collect::<Vec<Vec<i32>>>();

    let middle_numbers = valid_updates
        .iter()
        .map(|update| {
            let len = update.len();
            update[len / 2]
        })
        .collect::<Vec<i32>>();

    println!("Part one: {}", middle_numbers.iter().sum::<i32>());
}

fn part2() {
    let (rules, updates) = get_input();

    // if the order is not correct, move the first wrong number forwards in the array until it is correct
    let valid_updates = updates
        .iter()
        .map(|update| {
            let mut is_valid = check_validity(rules.clone(), update.clone());
            if is_valid.is_ok() {
                return Vec::new();
            }

            let mut updated = update.clone();

            while is_valid.is_err() {
                let (pos1, pos2) = is_valid.unwrap_err();
                updated.swap(pos1, pos2);
                is_valid = check_validity(rules.clone(), updated.clone());
            }

            updated
        })
        .collect::<Vec<Vec<i32>>>();

    let middle_numbers = valid_updates
        .iter()
        .filter(|p| p.len() > 0)
        .map(|update| {
            let len = update.len();
            update[len / 2]
        })
        .collect::<Vec<i32>>();

    println!("Part two: {}", middle_numbers.iter().sum::<i32>());
}

fn get_input() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let input = include_str!("../input.txt");
    let data = input.split("\n\n").collect::<Vec<&str>>();

    let rules = data[0]
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|rule| {
            rule.split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|p| p.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let updates = data[1]
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|p| p.trim().split(",").collect::<Vec<&str>>())
        .map(|p| {
            p.iter()
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (rules, updates)
}
