use crate::read_lines;

pub fn solve() {
    let lines = read_lines("input_day02");

    let total:u32 = lines
        .iter()
        .map(|line| {
            match line.split_once(" ") {
                Some(("A", "X")) => 3 + 1, // Draw + Rock
                Some(("A", "Y")) => 6 + 2, // Win  + Paper
                Some(("A", "Z")) => 0 + 3, // Loss + Scissors

                Some(("B", "X")) => 0 + 1, // Loss + Rock
                Some(("B", "Y")) => 3 + 2, // Draw + Paper
                Some(("B", "Z")) => 6 + 3, // Win  + Scissors

                Some(("C", "X")) => 6 + 1, // Win  + Rock
                Some(("C", "Y")) => 0 + 2, // Loss + Paper
                Some(("C", "Z")) => 3 + 3, // Draw + Scissors

                _ => 0	             // invalid
            }
        })
        .into_iter()
        .sum();

    println!("Total score after following the strategy: {}", total);
}
