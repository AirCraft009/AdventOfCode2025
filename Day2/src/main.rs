use std::env::split_paths;
use std::ffi::c_ulonglong;
use std::fs::File;
use std::io::{BufReader, Read, Split};

fn main() {
    let input : &mut String = &mut String::new();
    BufReader::new(File::open("input.txt").unwrap()).read_to_string(input).expect("could not read file");
    let ranges = input.split(",").collect::<Vec<&str>>();
    let mut sum: c_ulonglong = 0;
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
                println!("{}", active);
                sum += active.parse::<c_ulonglong>().unwrap();
            }
        }
    });
    println!("{}", sum)

}
