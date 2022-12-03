use crate::read_lines;

fn prio(c: char) -> u32 {
    (c as u32) - if c.is_ascii_lowercase() { 96 } else { 38 }
}

fn contains(s: &str, c: char) -> bool {
    s.chars().position(|c2| c == c2).is_some()
}

pub fn solve() {
    let lines = read_lines("data/input_day03");

    let total:u32 = lines
	.iter()
	.map(|line| {
	    let (left, right) = line.split_at(line.len() / 2);
            
            for c1 in left.chars() {
                if contains(right, c1) {
                    return Some(c1);
                }
            }

            None
	})
        .map(|dup| dup.map(prio).unwrap_or(0))
        .into_iter()
        .sum();

    println!("Sum of the priorities: {}", total);

    let s:u32 = lines
        .chunks_exact(3)
        .map(|chunk| {
            for c1 in chunk[0].chars() {
               if contains(&chunk[1], c1) && contains(&chunk[2], c1) {
                   return Some(c1);
               }
            }

            None
        })
        .map(|dup| dup.map(prio).unwrap_or(0))
        .into_iter()
        .sum();

    println!("Sum of the priorities per group: {}", s);
}
