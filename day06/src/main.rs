use std::fmt::Debug;

fn main() {
    part1();
}

fn get_input() -> Vec<Vec<&'static str>> {
    let input = include_str!("../input.txt");
    input
        .trim()
        .lines()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

#[derive(Debug, Clone)]
enum GuardDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct GuardPosition {
    x: i32,
    y: i32,
    direction: GuardDirection,
}

impl GuardPosition {
    fn new(x: i32, y: i32, direction: GuardDirection) -> GuardPosition {
        GuardPosition { x, y, direction }
    }
}

const GUARD_CHARACTERS: [&str; 4] = ["^", "v", ">", "<"];

fn get_guard_position(input: Vec<Vec<&str>>) -> GuardPosition {
    let y = input
        .iter()
        .position(|line| line.iter().any(|c| GUARD_CHARACTERS.contains(&c)))
        .unwrap() as i32;

    let x = input[y as usize]
        .iter()
        .position(|&letter| GUARD_CHARACTERS.contains(&letter))
        .unwrap() as i32;

    let guard_direction = match input[y as usize][x as usize] {
        "^" => GuardDirection::North,
        "v" => GuardDirection::South,
        ">" => GuardDirection::East,
        "<" => GuardDirection::West,
        _ => unreachable!(),
    };

    GuardPosition::new(x, y, guard_direction)
}

fn get_guard_next_position(input: Vec<Vec<&str>>) -> Result<GuardPosition, ()> {
    let guard_position = get_guard_position(input.clone());

    match guard_position.direction {
        GuardDirection::North => {
            if guard_position.y == 0 {
                return Err(());
            }

            Ok(GuardPosition::new(
                guard_position.x,
                guard_position.y - 1,
                guard_position.direction,
            ))
        }
        GuardDirection::East => {
            if guard_position.x == input[0].len() as i32 - 1 {
                return Err(());
            }

            Ok(GuardPosition::new(
                guard_position.x + 1,
                guard_position.y,
                guard_position.direction,
            ))
        }
        GuardDirection::South => {
            if guard_position.y == input.len() as i32 - 1 {
                return Err(());
            }

            Ok(GuardPosition::new(
                guard_position.x,
                guard_position.y + 1,
                guard_position.direction,
            ))
        }
        GuardDirection::West => {
            if guard_position.x == 0 {
                return Err(());
            }

            Ok(GuardPosition::new(
                guard_position.x - 1,
                guard_position.y,
                guard_position.direction,
            ))
        }
    }
}

fn next_direction(direction: GuardDirection) -> GuardDirection {
    match direction {
        GuardDirection::North => GuardDirection::East,
        GuardDirection::East => GuardDirection::South,
        GuardDirection::South => GuardDirection::West,
        GuardDirection::West => GuardDirection::North,
    }
}

fn part1() {
    let mut input = get_input();

    let mut guard_position = get_guard_position(input.clone());
    let mut next_position_result = get_guard_next_position(input.clone());

    while let Ok(mut next_pos) = next_position_result {
        let letter = input[next_pos.y as usize][next_pos.x as usize];

        if letter == "#" {
            next_pos.direction = next_direction(next_pos.direction);
            next_pos.x = guard_position.x;
            next_pos.y = guard_position.y;
        }

        input[guard_position.y as usize][guard_position.x as usize] = "x";
        input[next_pos.y as usize][next_pos.x as usize] =
            GUARD_CHARACTERS[next_pos.direction.clone() as usize];

        guard_position = next_pos;
        next_position_result = get_guard_next_position(input.clone());
    }

    input
        .iter()
        .map(|x| x.join(""))
        .collect::<Vec<String>>()
        .iter()
        .for_each(|x| println!("{}", x));

    let joined_input = input
        .iter()
        .map(|x| x.join(""))
        .collect::<Vec<String>>()
        .join("\n");

    println!(
        "Part one: {}",
        joined_input.chars().filter(|&c| c == 'x').count()
    );
}
