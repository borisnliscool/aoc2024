fn main() {
    part1();
    part2();
}

fn part1() {
    let mut count = 0;
    let lines = get_lines();

    let total_lines = lines.len();

    for y in 0..total_lines {
        let line = lines[y].clone();
        let line_len = line.len();

        for x in 0..line_len {
            let char = line[x];

            if x + 3 < line_len {
                if char == 'X' && line[x + 1] == 'M' && line[x + 2] == 'A' && line[x + 3] == 'S' {
                    count += 1;
                }

                if char == 'S' && line[x + 1] == 'A' && line[x + 2] == 'M' && line[x + 3] == 'X' {
                    count += 1;
                }
            }

            if y + 3 < total_lines {
                if char == 'X'
                    && lines[y + 1][x] == 'M'
                    && lines[y + 2][x] == 'A'
                    && lines[y + 3][x] == 'S'
                {
                    count += 1;
                }

                if char == 'S'
                    && lines[y + 1][x] == 'A'
                    && lines[y + 2][x] == 'M'
                    && lines[y + 3][x] == 'X'
                {
                    count += 1;
                }
            }

            if x + 3 < line_len && y + 3 < total_lines {
                if char == 'X'
                    && lines[y + 1][x + 1] == 'M'
                    && lines[y + 2][x + 2] == 'A'
                    && lines[y + 3][x + 3] == 'S'
                {
                    count += 1;
                }

                if char == 'S'
                    && lines[y + 1][x + 1] == 'A'
                    && lines[y + 2][x + 2] == 'M'
                    && lines[y + 3][x + 3] == 'X'
                {
                    count += 1;
                }
            }

            if x > 2 && y < total_lines - 3 {
                if char == 'X'
                    && lines[y + 1][x - 1] == 'M'
                    && lines[y + 2][x - 2] == 'A'
                    && lines[y + 3][x - 3] == 'S'
                {
                    count += 1;
                }

                if char == 'S'
                    && lines[y + 1][x - 1] == 'A'
                    && lines[y + 2][x - 2] == 'M'
                    && lines[y + 3][x - 3] == 'X'
                {
                    count += 1;
                }
            }
        }
    }

    println!("Part one: {}", count);
}

fn part2() {
    let lines = get_lines();

    let mut count = 0;

    let total_lines = lines.len();

    for y in 1..total_lines - 1 {
        let line = lines[y].clone();
        let line_len = line.len();

        for x in 1..line_len - 1 {
            let char = line[x];

            if char != 'A' {
                continue;
            }

            let top_left_char = lines[y - 1][x - 1];
            let top_right_char = lines[y - 1][x + 1];
            let bottom_left_char = lines[y + 1][x - 1];
            let bottom_right_char = lines[y + 1][x + 1];

            if (top_left_char != 'M' && top_left_char != 'S')
                || (top_right_char != 'M' && top_right_char != 'S')
                || (bottom_left_char != 'M' && bottom_left_char != 'S')
                || (bottom_right_char != 'M' && bottom_right_char != 'S')
            {
                continue;
            }

            if top_left_char == bottom_right_char || top_right_char == bottom_left_char {
                continue;
            }

            count += 1;
        }
    }

    println!("Part two: {}", count);
}

fn get_lines() -> Vec<Vec<char>> {
    include_str!("../input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
