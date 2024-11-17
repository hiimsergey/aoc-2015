const INPUT: &str = include_str!("3.txt");

fn main() { one(); two(); }

fn one() {
    let mut houses = process_houses(0..INPUT.len());
    houses.sort();
    houses.dedup();

    let result = houses.len();

    println!("{result}");
}

fn two() {
    let mut all_houses = [
        process_houses((0..INPUT.len()).filter(|n| n % 2 == 0)),
        process_houses((0..INPUT.len()).filter(|n| n % 2 == 1))
    ].concat();
    all_houses.sort();
    all_houses.dedup();

    let result = all_houses.len();

    println!("{result}");
}

// Takes an iterator of indices and returns a vector of coordinates after the respective
// amount of instructions
fn process_houses<T: IntoIterator<Item = usize>>(indices: T) -> Vec<(i32, i32)> {
    let mut houses = vec![(0, 0)];

    for i in indices {
        let (x_last, y_last) = *houses.last().unwrap();

        houses.push(
            match INPUT.chars().nth(i).unwrap() {
                '^' => (x_last, y_last + 1),
                'v' => (x_last, y_last - 1),
                '<' => (x_last - 1, y_last),
                /* '>' */ _ => (x_last + 1, y_last)
            }
        );
    }

    houses
}
