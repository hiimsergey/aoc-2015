use md5;

const INPUT: &str = include_str!("../../4.txt");

fn main() { one(); two(); }

fn one() {
    // TODO FALSE
    // From the given examples, it seems like the target number has at least
    // six digits.
    let mut result: u32 = 100_000;

    // I'm just gonna endlessly loop because a valid solution along the way is
    // guaranteed.
    loop {
        let key = format!("{INPUT}{result}");
        let hash = md5::compute(key);

        // `hash.0` is a u8-array representing the hash in decimal.
        // The first four zeros are 00 and 00, 0 and 0 and decimal.
        // The fifth digit is part of another array element. The first digit
        // of the u8 should therefore be smaller than the base, here 16.
        if hash.0[0..2] == [0, 0] && hash[2] < 16 { break; }
        result += 1;
    }

    print!("4.1: {result}\t\t");
}

fn two() {
    let mut result: u32 = 100_000;

    loop {
        let key = format!("{INPUT}{result}");
        let hash = md5::compute(key);

        // Six zeroes -> 00 00 00 -> [0, 0, 0] in decimal
        if hash.0[0..3] == [0, 0, 0] { break; }
        result += 1;
    }

    println!("4.2: {result}");
}
