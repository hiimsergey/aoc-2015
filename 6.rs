const INPUT: &str = include_str!("6.txt");

enum Signal { On, Off, Toggle }


fn main() { one(); two(); }

fn one() {
    let mut lights = [false; 1_000_000];

    for command in INPUT.lines() {
        // Char 6 is the first one that differs in all signals
        let signal = match command.chars().nth(6).unwrap() {
            'n' => Signal::On,
            'f' => Signal::Off,
            _ => Signal::Toggle
        };

        // god forgive me ...
        let mut words = command.split_whitespace().filter(|word|
            !["turn", "on", "off", "toggle", "through"].contains(word)
        );
        let mut start_coord = words.next().unwrap().split(',');
        let mut end_coord = words.next().unwrap().split(',');
        let sx: usize = start_coord.next().unwrap().parse().unwrap();
        let sy: usize = start_coord.next().unwrap().parse().unwrap();
        let ex: usize = end_coord.next().unwrap().parse().unwrap();
        let ey: usize = end_coord.next().unwrap().parse().unwrap();

        for y in sy..=ey {
            for x in sx..=ex {
                let i: usize = y * 1_000 + x;
                lights[i] = match signal {
                    Signal::On => true,
                    Signal::Off => false,
                    Signal::Toggle => !lights[i]
                };
            }
        }
    }

    let result = lights.iter().filter(|&&b| b).count();
    println!("{result}");
}

fn two() {
    let mut lights = [0_usize; 1_000_000];

    for command in INPUT.lines() {
        // Char 6 is the first one that differs in all signals
        let signal = match command.chars().nth(6).unwrap() {
            'n' => Signal::On,
            'f' => Signal::Off,
            _ => Signal::Toggle
        };

        // god forgive me ...
        let mut words = command.split_whitespace().filter(|word|
            !["turn", "on", "off", "toggle", "through"].contains(word)
        );
        let mut start_coord = words.next().unwrap().split(',');
        let mut end_coord = words.next().unwrap().split(',');
        let sx: usize = start_coord.next().unwrap().parse().unwrap();
        let sy: usize = start_coord.next().unwrap().parse().unwrap();
        let ex: usize = end_coord.next().unwrap().parse().unwrap();
        let ey: usize = end_coord.next().unwrap().parse().unwrap();

        for y in sy..=ey {
            for x in sx..=ex {
                let i: usize = y * 1_000 + x;
                match signal {
                    Signal::On => lights[i] += 1,
                    Signal::Off => if lights[i] != 0 { lights[i] -= 1 },
                    Signal::Toggle => lights[i] += 2
                };
            }
        }
    }

    let result: usize = lights.iter().sum();
    println!("{result}");
}
