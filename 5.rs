const INPUT: &str = include_str!("5.txt");

fn main() { one(); two(); }

fn one() {
    let result = INPUT
        .lines()
        .filter(|line| {
            !has_forbidden_sequences(line)
            && has_double_letters(line)
            && has_three_vowels(line)
        })
        .count();

    println!("{result}");
}

fn has_three_vowels(line: &str) -> bool {
    line
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(&c))
        .count()
        >= 3
}

fn has_double_letters(line: &str) -> bool {
    for i in 1..line.len() {
        if line.chars().nth(i - 1).unwrap() == line.chars().nth(i).unwrap() {
            return true;
        }
    }

    false
}

fn has_forbidden_sequences(line: &str) -> bool {
    for seq in ["ab", "cd", "pq", "xy"] {
        if line.contains(seq) { return true; }
    }

    false
}

fn two() {
    let result = INPUT
        .lines()
        .filter(|line| {
            has_two_letter_pairs(line)
            && has_repeating_letter(line)
        })
        .count();

    println!("{result}");
}

fn has_two_letter_pairs(line: &str) -> bool {
    for i in 0..(line.len() - 3) {
        if line[i + 2..].contains(&line[i..=i + 1]) { return true; }
    }

    false
}

fn has_repeating_letter(line: &str) -> bool {
    for i in 0..(line.len() - 2) {
        if line.chars().nth(i) == line.chars().nth(i + 2) {
            return true;
        }
    }

    false
}
