use crate::read_lines;

pub fn solve() {
    let lines = read_lines("data/input_day03");

    let total:u32 = lines
	.iter()
	.map(|line| {
	    let (left, right) = line.split_at(line.len() / 2);
            
            for c1 in left.chars() {
                if let Some(p) = right.chars().position(|c2| c1 == c2) {
                    return Some(c1);
                }
            }

            None
	})
        .map(|dup| {
            dup.map(|c| (c as u32) - if c.is_ascii_lowercase() { 96 } else { 38 }).unwrap_or(0)
        })
        .into_iter()
        .sum();

    println!("Sum of the priorities: {}", total);
}
