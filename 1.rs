// TODO END ALL CHECK correct string imports
const INPUT: &str = include_str!("1.txt");

fn main() { one(); two(); }

fn one() {
    let result: i32 = INPUT.chars().map(|c| if c == '(' { 1 } else { -1 }).sum();
    print!("1.1: {result}\t\t\t");
}

fn two() {
    let result = count_stories(0, 0);
    println!("1.2: {result}");
}

fn count_stories(index: usize, counter: i32) -> usize {
    if counter == -1 { return index; }
    
    // Since the input guarantees that the counter will turn -1 before the input
    // ends, I don't need to check the result and thus just unwrap.
    match INPUT.chars().nth(index).unwrap() {
        '(' => count_stories(index + 1, counter + 1),
        _ /* ')' */ => count_stories(index + 1, counter - 1)
    }
}
