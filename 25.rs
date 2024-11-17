const INPUT: &str = "To continue, please consult the code grid in the manual.  Enter the code at row 2947, column 3029.";

fn main() {
    // Filters the slice for both given coordinates
    let (line, col): (usize, usize) = {
        let coords_vec: Vec<usize> = INPUT
            .chars()
            .filter(|&c| c.is_digit(10) || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        (coords_vec[0], coords_vec[1])
    };
    
    // Initial code at line 1, column 1
    let mut result = 20_151_125;

    for _ in 0..get_number_of_transformations(line, col) {
        result = transform(result);
    }

    println!("{result}");
}

// Counts the number of coordinates between (1, 1) and (row, col)
// Basically, it generates this list:
// (1, 1) (2, 1) (1, 2) (3, 1) (2, 2) (1, 3) (4, 1) (3, 2) (2, 3) (3, 4) ...
// ... and returns the index of (row, col)
fn get_number_of_transformations(row: usize, col: usize) -> usize {
    let mut count = 0;

    // "round" refers to the group of codes between (x, 1) and (1, x)
    for round in 1.. {
        for coord in 0..round {
            if (round - coord, coord + 1) == (row, col) { return count; }
            else { count += 1; }
        }
    }

    panic!("Miraculously escaped infinite loop");
}

fn transform(code: usize) -> usize { code * 252_533 % 33_554_393 }
