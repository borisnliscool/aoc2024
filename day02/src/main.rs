fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input.txt");
    let reports = input.lines();

    let mut safe_reports = input.lines().count();

    for report in reports {
        let nums = report
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let is_same_direction =
            nums.windows(2).all(|w| w[0] <= w[1]) || nums.windows(2).all(|w| w[0] >= w[1]);
        if !is_same_direction {
            safe_reports -= 1;
            continue;
        }

        for i in 0..nums.len() - 1 {
            let current = nums[i];
            let next = nums[i + 1];

            if current == next {
                safe_reports -= 1;
                break;
            }

            let distance = (current - next).abs();
            if !(1..=3).contains(&distance) {
                safe_reports -= 1;
                break;
            }
        }
    }

    println!("Part one safe reports: {}", safe_reports);
}

fn safe_line(list: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = list.windows(2).map(|pair| pair[1] - pair[0]).collect();

    diffs.iter().all(|&d| d >= -3 && d < 0) || diffs.iter().all(|&d| d > 0 && d <= 3)
}

fn part2() {
    let input = include_str!("../input.txt");
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let safe_reports = reports.iter().filter(|x| {
        if safe_line(x.clone()) {
            return true;
        }

        for i in 0..x.len() {
            let mut removed = x.clone().clone();
            removed.remove(i);

            if safe_line(&removed) {
                return true;
            }
        }

        false
    });

    println!("Part two safe reports: {}", safe_reports.count());
}
