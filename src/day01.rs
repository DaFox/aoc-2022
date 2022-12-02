use crate::read_lines;

pub fn solve() {
    let lines = read_lines("data/input_day01");

    let sums = lines
	.split(|s| (*s).is_empty())
	.map(|g| (*g).iter().map(|s| s.parse::<u32>().unwrap()).sum::<u32>());
   
    let top1 = sums.clone().fold(0, |acc,val| if val > acc { val } else { acc });
    println!("Sum of most calories: {}", &top1);

    let top3 = sums.fold((0,0,0), |(a,b,c),val| if val > a { (val,a,b) } else if val > b { (a,val,b) } else if val > c { (a,b,val) } else { (a,b,c) } );
    println!("Sum of most top-3 calories: {}", top3.0 + top3.1 + top3.2);

}
