const INPUT: &str = include_str!("2.txt");

fn main() { one(); two(); }

fn one() {
    let result: u32 = INPUT.lines().map(|dimension| {
        let lwh: Vec<u32> = dimension.split('x').map(|n| n.parse().unwrap()).collect();
        let (lw, wh, hl) = (lwh[0] * lwh[1], lwh[1] * lwh[2], lwh[2] * lwh[0]);

        2 * lw + 2 * wh + 2 * hl + lw.min(wh).min(hl)
    }).sum();

    println!("{result}");
}

fn two() {
    let result: u32 = INPUT.lines().map(|dimension| {
        let mut lwh: Vec<u32> = dimension.split('x').map(|n| n.parse().unwrap()).collect();
        lwh.sort();

        2 * lwh[0] + 2 * lwh[1] + lwh.iter().product::<u32>()
    }).sum();

    println!("{result}");
}
