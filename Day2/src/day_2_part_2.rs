use std::ffi::c_ulonglong;
use std::fs::File;
use std::io::{BufReader, Read};

pub(crate) fn day_2_part_2() -> u64 {
    let input : &mut String = &mut String::new();
    BufReader::new(File::open("input.txt").unwrap()).read_to_string(input).expect("could not read file");
    let ranges = input.split(",").collect::<Vec<&str>>();
    let mut sum: u64 = 0;
    ranges.iter().for_each(|range| {
        let range = range.split("-").collect::<Vec<&str>>();
        let start = range[0].parse::<usize>().unwrap();
        let end = range[1].parse::<usize>().unwrap();

        for i in (start..=end) {
            // PLan
            // String has to start with char sequence
            // if char seq.len % string.len != 0 this ammount of chars can't be repeated so skip
            let str_num = i.to_string();
            // if the len of the char seq is more than half it can't fit inside of the string
            for j in (1..=str_num.len()/2){
                if str_num.len() % j != 0{
                    continue;
                }
                let str_sequence : &str = &str_num[ ..j];

                let mut fin_str: String = String::new();
                // str_num.len()/j can't have any size because of str_num.len % j
                for _ in 0 .. str_num.len()/j{
                    fin_str.push_str(str_sequence);
                }

                if fin_str == str_num {
                    sum += i as u64;
                    break;
                }
            }
        }
    });
    sum
}