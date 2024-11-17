const INPUT: &str = include_str!("23.txt");

fn main() { one(); two(); }

fn one() {
    let result = read(0, 0, 0);
    print!("23.1: {result}\t\t");
}

fn two() {
    let result = read(0, 1, 0);
    println!("23.2: {result}");
}

// Reads the index-th line of the instructions and alters registers (a and b)
fn read(index: isize, a: isize, b: isize) -> isize {
    match INPUT.lines().nth(index as usize) {
        Some(line) => {
            let args: Vec<&str> = line.split_whitespace().collect();

            match args[0] {
                "hlf" => if args[1] == "a" {
                    read(index + 1, a / 2, b)
                } else {
                    read(index + 1, a, b / 2)
                },

                "tpl" => if args[1] == "a" {
                    read(index + 1, a * 3, b)
                } else {
                    read(index + 1, a, b * 3)
                },

                "inc" => if args[1] == "a" {
                    read(index + 1, a + 1, b)
                } else {
                    read(index + 1, a, b + 1)
                },

                "jmp" => read(index + args[1].parse::<isize>().unwrap(), a, b),

                "jie" => if args[1] == "a," && a % 2 == 0
                || args[2] == "b," && b % 2 == 0 {
                    read(index + args[2].parse::<isize>().unwrap(), a, b)
                } else { read(index + 1, a, b) },

                /* "jio" */ _ => if args[1] == "a," && a == 1
                || args[1] == "b," && b == 1 {
                    read(index + args[2].parse::<isize>().unwrap(), a, b)
                } else { read(index + 1, a, b) }
            }
        },
        None => b
    }
}
