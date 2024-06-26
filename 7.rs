use std::collections::HashMap;

type Assignments<'a> = HashMap<&'a str, &'a str>;

const INPUT: &str = include_str!("7.txt");

fn main() { one(); two(); }

fn one() {
    let mut map = Assignments::new();
    
    for line in INPUT.lines() {
        let mut line = line.split(" -> ");
        let from = line.next().unwrap();
        let into = line.next().unwrap();
        map.insert(into, from);
    }

    let mut level: i16 = 1;
    let result = find_signal("a", &map, 1, &mut level);
    println!("{result}");
}

fn find_signal(of: &str, map: &Assignments, cur: i16, level: &mut i16) -> i32 {
    if cur > *level { *level = cur; }
    println!("TODO: {level} - level of find_signal");

    if let Ok(sig) = of.parse() { return sig; }

    let gate: Vec<&str> = map.get(&of).unwrap().split_whitespace().collect();

    match gate.len() {
        // must be "<wire>"
        1 => return find_signal(gate[0], map, cur + 1, level),
        
        // must be "NOT <wire>"
        2 => return -find_signal(gate[1], map, cur + 1, level),

        // must be "<wire> AND/OR/RSHIFT/LSHIFT <wire>"
        _ => {
            let source_1 = find_signal(gate[0], map, cur + 1, level);
            let source_2 = find_signal(gate[2], map, cur + 1, level);
            return match gate[1] {
                "AND" => source_1 & source_2,
                "OR" => source_1 | source_2,
                "LSHIFT" => source_1 << source_2,
                 _ => source_1 >> source_2
            };
        }
    }
}

fn two() {
    todo!()
}
