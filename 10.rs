const INPUT: &str = include_str!("10.txt");

fn main() { one(); two(); }

fn one() {
    // TODO FALSE
    let mut input_vec: Vec<u8> = INPUT.chars().map(|s| s.to_digit(10).unwrap() as u8).collect();
    for _ in 0..40 { input_vec = look_and_say(&input_vec); }

    let result = input_vec.len();
    println!("{result}");
}

fn look_and_say(input: &Vec<u8>) -> Vec<u8> {
    let mut result = vec![];
    let mut occurences = 0;
    let mut current = input[0];

    for n in input {
        if current != *n {
            result.push(occurences);
            result.push(current);
            occurences = 1;
            current = *n;
        } else {
            occurences += 1;
        }
    }

    result.push(occurences);
    result.push(current);
    result
}

fn two() {
    let mut input: Vec<u8> = INPUT.chars().map(|s| s.to_digit(10).unwrap() as u8).collect();
    for _ in 0..50 { input = look_and_say(&input); }

    let result = input.len();
    println!("{result}");
}
