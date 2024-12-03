use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input.txt");

    let re = Regex::new(r"mul\((\d{1,3}),\s?(\d{1,3})\)").unwrap();
    let mut sum = 0;

    re.captures_iter(input).for_each(|cap| {
        let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += x * y
    });

    println!("Part one: {}", sum);
}

fn part2() {
    let input = include_str!("../input.txt");
    let data = input.lines().collect::<Vec<&str>>().join("");

    let re = Regex::new(r"mul\((\d{1,3}),\s?(\d{1,3})\)").unwrap();
    let split = data
        .split("do()")
        .collect::<Vec<&str>>()
        .iter()
        .map(|section| {
            let re = Regex::new(r"don't\(\).*").unwrap();
            re.replace_all(section, "").to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    let mut sum = 0;
    re.captures_iter(&*split).for_each(|cap| {
        let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        sum += x * y
    });

    println!("Part two: {}", sum);
}
