pub mod day01;
pub mod day02;
pub mod day03;

pub fn read_lines(path: &str) -> Vec<String> {
    let input = std::fs::read(path)
        .expect("Unable to open input file");

    std::str::from_utf8(&input)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

