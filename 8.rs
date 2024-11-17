const INPUT: &str = include_str!("8.txt");

fn main() { one(); two(); }

fn one() {
    let result: usize = INPUT.lines().map(|line| {
        line.len() - chars_in_mem(&line[1..line.len() - 1], 0)
    }).sum();
    print!("8.1: {result}\t\t");
}

fn two() {
    let result: usize = INPUT.lines().map(|line| {
        encoded_str_len(line) - line.len()
    }).sum();
    println!("8.2: {result}");
}

fn chars_in_mem(slice: &str, counter: usize) -> usize {
    match slice.chars().next() {
        Some('\\') => if slice.chars().nth(1).unwrap() == 'x' {
            // char sequence following -> \x27...
            //     passing slice from here on ^
            chars_in_mem(&slice[4..], counter + 1)
        } else {
            // special char following -> \\...
            //  passing slice from here on ^
            chars_in_mem(&slice[2..], counter + 1)
        },
        // just omitting the first char
        Some(_) => chars_in_mem(&slice[1..], counter + 1),
        None => counter
    }
}

fn encoded_str_len(slice: &str) -> usize {
    // + 2 for the additional pair of double quotes
    slice.replace('\\', "\\\\").replace('\"', "\\\"").len() + 2
}
