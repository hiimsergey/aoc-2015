const INPUT: &str = include_str!("24.txt");

fn main() { one(); two(); }

fn one() {
    let mut pkgs: Vec<u16> = INPUT
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let pkgs_len: u32 = pkgs.len() as u32;

    let mut position: u16 = 0;
    while position < 3u16.pow(pkgs_len) {
        for i in 0..pkgs_len {
            println!("{i}: {}", position / 3u16.pow(i) % 3);
        }

        position += 1;
    }
}

fn two() {
    todo!()
}
// TODO
