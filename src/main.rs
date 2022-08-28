use std::fs;

mod day1;

fn day1() {
    println!("Day 1");
    let input = fs::read_to_string("./res/input-day1.txt")
        .expect("Unable to read input day 1");
    println!("part 1 - The Easter bunny is {} blocks away.", day1::part1(&input).unwrap());
    println!("part 2 - The Easter bunny is {} blocks away.", day1::part2(&input).unwrap());
}

fn main() {
    day1();
}
