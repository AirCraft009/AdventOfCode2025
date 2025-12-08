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
}



impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }

    fn inbetween(self, num: u64) -> bool {
        self.start <= num && num <= self.end
    }


}