use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug,Clone)]
enum TargetId {
    Bot(u8),
    Output(u8),
}

#[derive(Debug,Clone)]
struct Bot {
    value_1: Option<u8>,
    value_2: Option<u8>,
    target_id_low: TargetId,
    target_id_high: TargetId,
    test_low : u8,
    test_high : u8,
    processed : bool,
}

impl Bot {
    fn new(target_id_low : TargetId, target_id_high : TargetId, test_low : u8, test_high : u8) -> Bot {
        Bot {
            value_1 : None,
            value_2 : None,
            target_id_low,
            target_id_high,
            test_low,
            test_high,
            processed : false
        }
    }

    fn accept(&mut self, value : u8) {
        // store the value as value_1 or value_2
        if self.value_1.is_none() {
            self.value_1 = Some(value);
        } else if self.value_2.is_none() {
            self.value_2 = Some(value);
        } else {
            panic!("Cannot accept value {} on bot {:?}", value, self);
        }

        // Place value_1 and value_2 in the right order (low, high)
        if self.value_1.is_some() && self.value_2.is_some() {
            // get low and high value from the bot in the right order
            let (mut value_low, mut value_high) = (self.value_1.unwrap(), self.value_2.unwrap());
            if value_low > value_high {
                (value_low, value_high) = (value_high, value_low);
            }

            self.value_1 = Some(value_low);
            self.value_2 = Some(value_high);

            // Update to record if right values have been processed
            self.processed = self.test_low == value_low && self.test_high == value_high;
        }
    }
}

fn factory(input: &Vec<String>, test_low : u8, test_high : u8) -> (Vec<(u8,u8)>, HashMap<u8,Bot>,) {
    lazy_static! {
            static ref VALUE: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
            static ref MOVE: Regex = Regex::new(r"bot (\d+) gives low to (bot (\d+)|output (\d+)) and high to (bot (\d+)|output (\d+))").unwrap();
        }

    let mut bots : HashMap<u8,Bot> = HashMap::new();
    let mut values : Vec<(u8,u8)> = Vec::new();

    for line in input {
        if line.starts_with("value") {
            if let Some(caps) = VALUE.captures(line) {
                let value = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let bot_id = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
                values.push((value, bot_id));
            } else {
                panic!("Unable to parse 'value' line '{}'", line);
            }
        } else if line.starts_with("bot") {
            if let Some(caps) = MOVE.captures(line) {
                let id = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
                let target_id_low = if caps.get(2).unwrap().as_str().starts_with("bot") {
                    TargetId::Bot(caps.get(3).unwrap().as_str().parse::<u8>().unwrap())
                } else {
                    TargetId::Output(caps.get(4).unwrap().as_str().parse::<u8>().unwrap())
                };
                let target_id_high = if caps.get(5).unwrap().as_str().starts_with("bot") {
                    TargetId::Bot(caps.get(6).unwrap().as_str().parse::<u8>().unwrap())
                } else {
                    TargetId::Output(caps.get(7).unwrap().as_str().parse::<u8>().unwrap())
                };
                bots.insert(id, Bot::new(target_id_low, target_id_high, test_low, test_high));
            } else {
                panic!("Unable to parse 'move' line '{}'", line);
            }
        } else {
            panic!("Unable to parse line '{}'", line);
        }
    }

    (values, bots)
}


fn process(values: &Vec<(u8, u8)>, bots : &mut HashMap<u8, Bot>) -> HashMap<u8,u8>{
    let mut output= HashMap::new();

    for (value, bot_id) in values {
        // get a value and update the required bot
        bots.get_mut(bot_id).unwrap().accept(*value);

        // update all bots impacted, starting with the one accepted the value
        // use a queue as updates can impact new bots not impacted originally
        let mut updated = vec![*bot_id];
        while !updated.is_empty() {
            // Get the bot values required for updating targets. This cannot be done using a borrow
            // as the compiler cannot ensure the mutable borrowed bot won;t change during the
            // target update ... yes sometimes this ownership thing is not straight forward
            let (value_1, value_2, target_id_low, target_id_high) = {
                let b = bots.get(&updated.pop().unwrap()).unwrap();
                (b.value_1.clone(), b.value_2.clone(), b.target_id_low.clone(), b.target_id_high.clone())
            };

            // if the bot has two values ... then targets should be updated
            if value_1.is_some() && value_2.is_some() {
                let (value_low, value_high) = (value_1.unwrap(), value_2.unwrap());

                // Update targets
                for (id, value) in [(target_id_low, value_low), (target_id_high, value_high)] {
                    match id {
                        TargetId::Output(output_id) => {
                            output.insert(output_id, value);
                            () // required to ensure the match arms have the same result value
                        },
                        TargetId::Bot(bot_id) => {
                            bots.get_mut(&bot_id).unwrap().accept(value);
                            updated.push(bot_id);
                            () // required to ensure the match arms have the same result value
                        },
                    };
                }
            }
        }
    }

    output
}

pub fn part1(input: &Vec<String>) -> Result<u8,&str> {
    let (values, mut bots) = factory(input, 17, 61);
    let _ = process(&values, &mut bots);

    let (key, _) = bots.iter().find(|(_, v)| v.processed == true).unwrap();
    Ok(*key)
}

pub fn part2(input: &Vec<String>) -> Result<i32,&str> {
    let (values, mut bots) = factory(input, 17, 61);
    let output = process(&values, &mut bots);

    let a = *output.get(&0).unwrap() as i32;
    let b = *output.get(&1).unwrap() as i32;
    let c = *output.get(&2).unwrap() as i32;

    Ok(a * b * c)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::day10::{factory, process};

    #[rstest]
    fn test_process() {
        let input = vec![
            "value 5 goes to bot 2".to_string(),
            "bot 2 gives low to bot 1 and high to bot 0".to_string(),
            "value 3 goes to bot 1".to_string(),
            "bot 1 gives low to output 1 and high to bot 0".to_string(),
            "bot 0 gives low to output 2 and high to output 0".to_string(),
            "value 2 goes to bot 2".to_string()
        ];

        let (values, mut bots) = factory(&input, 2, 5);
        let output = process(&values, &mut bots);
        assert_eq!(output.get(&0).unwrap(), &5u8);
        assert_eq!(output.get(&1).unwrap(), &2u8);
        assert_eq!(output.get(&2).unwrap(), &3u8);
        assert_eq!(bots.get(&0).unwrap().processed, false);
        assert_eq!(bots.get(&1).unwrap().processed, false);
        assert_eq!(bots.get(&2).unwrap().processed, true);
    }
}
