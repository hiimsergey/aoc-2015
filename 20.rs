const INPUT: &str = include_str!("20.txt");

fn main() { one(); two(); }

fn one() {
    // I divide by 10 so that I don't have to multiply the number of presents by 10
    // for each house, thus saving computations.
    let target = INPUT.trim().parse::<usize>().unwrap() / 10;

    let result: usize = (1..).find(|&house| {
        let house_sqrt = (house as f32).sqrt() as usize;
        (1..=house_sqrt)
            // finds all divisors up to the square root of the house number
            .filter(|n| house % n == 0)
            // adds the counterpart `house / n`, thus saving computations
            .map(|n| n + house / n)
            .sum::<usize>()
        >= target
    }).unwrap();

    print!("20.1: {result}\t\t");
}

fn two() {
    // I divide by 11 so that I don't have to multiply the number of presents by 10
    // for each house, thus saving computations.
    let target = INPUT.trim().parse::<usize>().unwrap() / 11;

    let result: usize = (1..).find(|&house| {
        let house_sqrt = (house as f32).sqrt() as usize;
        (1..=house_sqrt)
            .filter(|n| house % n == 0)
            // The following two methods adds the counterpart of every divisor
            // to the vector.
            .map(|n| (n, house / n))
            .flat_map(|(n, house_div_n)| vec![n, house_div_n])
            // Lastly the divisors whose counterparts are over 50 are sorted out.
            .filter(|n| house / n <= 50)
            .sum::<usize>()
        >= target
    }).unwrap();

    println!("20.2: {result}");
}
