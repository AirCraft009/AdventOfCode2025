use std::fs::File;
use std::io::{BufReader, Read, Split};

pub(crate) fn day_2_part_1() -> u64{
    let input : &mut String = &mut String::new();
    BufReader::new(File::open("input.txt").unwrap()).read_to_string(input).expect("could not read file");
    let ranges = input.split(",").collect::<Vec<&str>>();
    let mut sum: u64 = 0;
    ranges.iter().for_each(|range| {
        let range = range.split("-").collect::<Vec<&str>>();
        let start = range[0].parse::<usize>().unwrap();
        let end = range[1].parse::<usize>().unwrap();

        for i in (start..=end) {
            let active = i.to_string();
            if active.len() % 2 != 0{
                continue;
            }

            if active[0..active.len()/2] == active[active.len()/2 .. active.len()]{
                sum += active.parse::<u64>().unwrap();
            }
        }
    });
    sum
}