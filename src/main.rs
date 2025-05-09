fn main() {
    let answer1 = part1(include_str!("input.txt"));
    println!("{}", answer1);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
            let first: u32 = numbers.iter().next().unwrap().to_digit(10).unwrap();
            let last = numbers.iter().next_back().unwrap().to_digit(10).unwrap();
            first * 10 + last
        })
        .sum()
}

#[test]
fn test1() {
    let example_part1 = 
    "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    assert_eq!(142, part1(example_part1));
}
