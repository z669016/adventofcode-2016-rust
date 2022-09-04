use chrono::{Duration, Local};
use crate::ioc::{line_from_file, lines_from_file};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod ioc;
mod day8;
mod day9;

fn duration(duration : Duration) -> String {
    format!("{:02}:{:02}:{:02}.{:03}"
            , duration.num_hours() % 24
            , duration.num_minutes() % 60
            , duration.num_seconds() % 60
            , duration.num_milliseconds() % 1000)
}

fn day1() {
    println!("Day 1");
    let input = line_from_file("./res/input-day1.txt");

    let start = Local::now();
    println!("part 1 - The Easter bunny is {} blocks away.", day1::part1(&input).unwrap());
    let step = Local::now();
    println!("part 2 - The Easter bunny is {} blocks away.", day1::part2(&input).unwrap());
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day2() {
    println!("Day 2");
    let input = lines_from_file("./res/input-day2.txt");

    let start = Local::now();
    println!("part 1 - The bathroom code is {}", day2::part1(&input));
    let step = Local::now();
    println!("part 2 - The bathroom code is {}", day2::part2(&input));
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day3() {
    println!("Day 3");
    let input = lines_from_file("./res/input-day3.txt");

    let start = Local::now();
    println!("part 1 - The number of possible triangles is {}", day3::part1(&input));
    let step = Local::now();
    println!("part 2 - The number of possible triangles vertically grouped is {}", day3::part2(&input));
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day4() {
    println!("Day 4");
    let input = lines_from_file("./res/input-day4.txt");

    let start = Local::now();
    println!("part 1 - Sum of the sector IDs of the real rooms is {}", day4::part1(&input));
    let step = Local::now();
    println!("part 2 - Room for North Pole objects has sector ID {}", day4::part2(&input).unwrap());
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day5() {
    println!("Day 5");
    let input = "abbhdwsy";

    let start = Local::now();
    println!("part 1 - The password is {:?}", day5::part1(&input));
    let step = Local::now();
    println!("part 2 - The password is {:?}", day5::part2(&input));
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day6() {
    println!("Day 6");
    let input = lines_from_file("./res/input-day6.txt");

    let start = Local::now();
    println!("part 1 - The error-corrected version of the message being sent is {:?}", day6::part1(&input));
    let step = Local::now();
    println!("part 2 - The original message that Santa is trying to send is {:?}", day6::part2(&input));
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day7() {
    println!("Day 7");
    let input = lines_from_file("./res/input-day7.txt");

    let start = Local::now();
    println!("part 1 - The number of IPs supporting TLS is {:?}", day7::part1(&input));
    let step = Local::now();
    println!("part 2 - The number of IPs supporting SSL is {:?}", day7::part2(&input));
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day8() {
    println!("Day 8");
    let input = lines_from_file("./res/input-day8.txt").iter()
        .map(|line| day8::Command::from(line).unwrap())
        .collect::<Vec<day8::Command>>();

    let start = Local::now();
    println!("part 1 - The number of pixels lit is {:?}", day8::part1(&input));
    let step = Local::now();
    println!("part 2 - The screen shows:");
    println!("{}", day8::part2(&input));
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn day9() {
    println!("Day 9");
    let input = line_from_file("./res/input-day9.txt");

    let start = Local::now();
    println!("part 1 - The size of the decrypted text is {:?}", day9::part1(&input).unwrap());
    let step = Local::now();
    println!("part 2 - The size of the decrypted text is {:?}", day9::part2(&input).unwrap());
    let end = Local::now();

    println!("Part 1 {:?}, and part 2 {:?}.", duration(step - start), duration(end - step));
}

fn main() {
    day1();
    println!();
    day2();
    println!();
    day3();
    println!();
    day4();
    println!();
    day5();
    println!();
    day6();
    println!();
    day7();
    println!();
    day8();
    println!();
    day9();
}
