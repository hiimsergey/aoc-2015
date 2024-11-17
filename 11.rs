const INPUT: &str = include_str!("11.txt");

fn main() { one(); two(); }

fn one() {
    // TODO FALSE
	let mut result = increment(INPUT.into());

	let mut todo = 0;
	while !(
		has_consecutive_letters(&result, b' ', 1)
		&& !['i', 'o', 'l'].iter().any(|c| result.contains(*c))
		&& has_two_double_letters(&result, ' ', 0)
	) {
		// TODO
		todo += 1;
		println!("{todo}");
		result = increment(result);
	}

	println!("{result}");
}

fn has_consecutive_letters(input: &str, prev: u8, series: u8) -> bool {
	if series == 3 { return true; }

	match input.chars().next() {
		Some(c) if c as u8 == prev + 1 => has_consecutive_letters(&input[1..], c as u8, series + 1),
		Some(c) => has_consecutive_letters(&input[1..], c as u8, 1),
		None => false
	}
}

fn has_two_double_letters(input: &str, prev: char, count: u8) -> bool {
	if count == 2 { return true; }

	match input.chars().next() {
		Some(c) if c == prev => has_two_double_letters(&input[1..], ' ', count + 1),
		Some(c) => has_two_double_letters(&input[1..], c, count),
		None => false
	}
}

fn increment(input: String) -> String {
	real_increment(input.chars().rev().collect()).chars().rev().collect()
}

fn real_increment(input: String) -> String {
	match input.chars().next().unwrap() {
		'z' => format!("a{}", increment(input[1..].into())),
		other => format!("{}{}", (other as u8 + 1) as char, &input[1..]),
	}
}

fn two() {
    todo!()
}
