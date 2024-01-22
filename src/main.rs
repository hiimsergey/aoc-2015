mod day1; mod day2; mod day3; mod day4; mod day5; mod day6; mod day7; mod day8; mod day9;
mod day10; mod day11; mod day12; mod day13; mod day14; mod day15; mod day16; mod day17;
mod day18; mod day19; mod day20; mod day21; mod day22; mod day23; mod day24;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: aoc2015 <DAY> <PART>");
        std::process::exit(1);
    }

    let (day, part) = (
        args[1].as_str(),
        if args.len() < 3 { "1" } else { args[2].as_str() }
    );

    match (day, part) {
        ("1", "1") => day1::one(), ("1", "2") => day1::two(),
        ("2", "1") => day2::one(), ("2", "2") => day2::two(),
        ("3", "1") => day3::one(), ("3", "2") => day3::two(),
        ("4", "1") => day4::one(), ("4", "2") => day4::two(),
        ("5", "1") => day5::one(), ("5", "2") => day5::two(),
        ("6", "1") => day6::one(), ("6", "2") => day6::two(),
        ("7", "1") => day7::one(), ("7", "2") => day7::two(),
        ("8", "1") => day8::one(), ("8", "2") => day8::two(),
        ("9", "1") => day9::one(), ("9", "2") => day9::two(),
        ("10", "1") => day10::one(), ("10", "2") => day10::two(),
        ("11", "1") => day11::one(), ("11", "2") => day11::two(),
        ("12", "1") => day12::one(), ("12", "2") => day12::two(),
        ("13", "1") => day13::one(), ("13", "2") => day13::two(),
        ("14", "1") => day14::one(), ("14", "2") => day14::two(),
        ("15", "1") => day15::one(), ("15", "2") => day15::two(),
        ("16", "1") => day16::one(), ("16", "2") => day16::two(),
        ("17", "1") => day17::one(), ("17", "2") => day17::two(),
        ("18", "1") => day18::one(), ("18", "2") => day18::two(),
        ("19", "1") => day19::one(), ("19", "2") => day19::two(),
        ("20", "1") => day20::one(), ("20", "2") => day20::two(),
        ("21", "1") => day21::one(), ("21", "2") => day21::two(),
        ("22", "1") => day22::one(), ("22", "2") => day22::two(),
        ("23", "1") => day23::one(), ("23", "2") => day23::two(),
        ("24", "1") => day24::one(), ("24", "2") => day24::two(),
        (_, _) => {
            eprintln!("Wrong arguments!\nFirst: 1-24\nSecond: 1-2");
            std::process::exit(1);
        }
    }
}
