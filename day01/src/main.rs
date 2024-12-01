use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    let input = include_str!("../input.txt");

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let split = line.split("   ").collect::<Vec<&str>>();
        list1.push(split[0].parse::<i32>().unwrap());
        list2.push(split[1].parse::<i32>().unwrap());
    }

    (list1, list2)
}

fn part1() {
    let (mut list1, mut list2) = get_lists();

    list1.sort();
    list2.sort();

    let mut sum: i32 = 0;

    for i in 0..list1.len() {
        let diff = (list1[i] - list2[i]).abs();
        sum += diff;
    }

    println!("Answer to part 1: {}", sum);
}

fn part2() {
    let (list1, list2) = get_lists();

    let mut hash_map = HashMap::new();

    for item in list2 {
        hash_map.insert(item, hash_map.get(&item).unwrap_or(&0) + 1);
    }

    let mut sum: i32 = 0;

    for item in list1 {
        let count = hash_map.get(&item).unwrap_or(&0);
        sum += item * count;
    }

    println!("Answer to part 2: {}", sum);
}