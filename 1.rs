// TODO COMMENT ALL

const INPUT: &str = include_str!("1.txt");

fn main() { one(); two(); }

fn one() {
    println!("--- 1.1 ---");
    let result: i16 = INPUT.chars().map(|c| match c {
        '(' => 1, _ => -1
    }).sum();
    println!("{result}");
}

fn two() {
    let result = count_stories(0, 0);
    println!("{result}");
}

fn count_stories(index: usize, counter: i16) -> usize {
    if counter == -1 { return index; }

    // Since the input guarantees that the counter will turn -1 before the file
    // ends, I don't need to check the result and thus just unwrap.
    match INPUT.chars().nth(index).unwrap() {
        '(' => count_stories(index + 1, counter + 1),
        /* ')' */ _ => count_stories(index + 1, counter - 1)
    }
}
