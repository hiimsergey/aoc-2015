const INPUT: &str = include_str!("6.txt");

fn main() { one(); two(); }

struct Light {
    line: usize,
    col: usize
}

type Command = Box<dyn FnMut(&mut bool)>;

fn one() {
    let mut lights = [false; 1_000_000];

    for line in INPUT.lines() {
        let (mut command, from, to) = parse_line_one(line);

        for line in from.line..=to.line {
            for col in get_col_iter(from.col, to.col).iter() {
                // Applies the actual work (turn on, turn off, toggle) to the light
                // line * 1_000 + col converts coordinates to linear index
                command(&mut lights[line * 1_000 + col]);
            }
        }
    }

    let result = lights.iter().filter(|b| **b).count();
    print!("6.1: {result}\t\t");
}

fn two() {
    let mut lights = [0; 1_000_000];

    for line in INPUT.lines() {
        let (light_level, from, to) = parse_line_two(line);

        for line in from.line..=to.line {
            for col in get_col_iter(from.col, to.col).iter() {
                // Applies the actual work (turn on, turn off, toggle) to the light
                // line * 1_000 + col converts coordinates to linear index
                let i = line * 1_000 + col;
                lights[i] += light_level;
                
                // The lights can't have negative light levels
                if lights[i] < 0 { lights[i] = 0; }
            }
        }
    }

    let result: i32 = lights.iter().sum();
    println!("6.2: {result}");
}

// Takes a line and returns the type of action as Command (turn on, turn off, toggle)
// and the both given coordinates.
fn parse_line_one(line: &'static str) -> (Command, Light, Light) {
    let command = move |b: &mut bool| match (line.contains("on"), line.contains("off")) {
        (true, _) => *b = true,
        (_, true) => *b = false,
        _ => *b = !*b
    };

    let mut coordinates = get_line_without_command(line)
            .split(" through ")
            .flat_map(|n| n.split(','))
            .map(|n| n.parse::<usize>().unwrap());
    
    (
        Box::new(command),
        Light {
            line: coordinates.next().unwrap(),
            col: coordinates.next().unwrap(),
        },
        Light {
            line: coordinates.next().unwrap(),
            col: coordinates.next().unwrap()
        }
    )
}

// Returns a slice of the line that starts with the first numeric character
// (thus leaving out the command part).
fn get_line_without_command(line: &str) -> &str {
    &line[line.chars().position(char::is_numeric).unwrap_or(line.len())..]
}

// Since the second given point can be on a column before the first one,
// it wouldn't be possible to iterate the coloumns between them. That's why we
// we need to check whether it's the case, so that we can "switch" them.
fn get_col_iter(a: usize, b: usize) -> Vec<usize> {
    if b < a { return (b..=a).rev().collect(); }
    (a..=b).collect()
}

// Takes a line and returns the light level to increase/decrease by and the both
// given coordinates.
fn parse_line_two(line: &'static str) -> (i32, Light, Light) {
    let light_level = match (line.contains("on"), line.contains("off")) {
        (true, _) => 1,
        (_, true) => -1,
        _ => 2
    };

    let mut coordinates = get_line_without_command(line)
            .split(" through ")
            .flat_map(|n| n.split(','))
            .map(|n| n.parse::<usize>().unwrap());
    
    (
        light_level,
        Light {
            line: coordinates.next().unwrap(),
            col: coordinates.next().unwrap(),
        },
        Light {
            line: coordinates.next().unwrap(),
            col: coordinates.next().unwrap()
        }
    )
}
