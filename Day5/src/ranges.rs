use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};
use std::os::raw::c_int;

#[derive(Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

struct Ranges {
    ranges: Vec<Range>,
    // the absolute max and mins if the number isn't inbetween these it's guaranteed to not be in any range
    max: u64,
    min: u64,
    // ammount of fresh items
    fresh: u64,
}



impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }

    fn inbetween(self, num: u64) -> bool {
        self.start <= num && num <= self.end
    }
}

impl Ranges {
    fn new(file: &str) -> Ranges {
        let str_ranges: &mut String = &mut String::new();
        let _ = BufReader::new(File::open(file).unwrap()).read_to_string(str_ranges).unwrap();
        let mut ranges = Ranges{ranges: vec![], max: 0, min: 0, fresh: 0};
        ranges.parse(str_ranges);
    }

    fn parse(&mut self, parse: &mut String) {
        let splits = parse.split("\n\n").collect::<Vec<&str>>();
        let str_ranges = splits[0];
        let items = splits[1];
        for str_range in str_ranges.lines() {
            let start_end = str_range.split("-").collect::<Vec<&str>>();
            let start = start_end[0].parse::<u64>().unwrap();
            let end = start_end[1].parse::<u64>().unwrap();
            let range = Range::new(start, end);
            self.ranges.push(range);
        }

    }
}