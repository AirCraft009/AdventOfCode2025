use std::cmp::PartialEq;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

const LIMIT: u16 = 100;
pub(crate) struct Lock {
    pub(crate) active_num: u8,
    pub(crate) null_counter: usize,
    pub(crate) null_passed_counter: usize,
    reader: BufReader<File>,
}

fn wrapping_add_limit(x: u16, y: u16, limit: u16) -> (u16, u16) {
    let times = y/limit;
    let times = if (y%limit + x) > limit { times + 1} else {times};
    let r = ((x as u32 + y as u32) % limit as u32) as u16;
    let times = if (r%limit == 0) && times > 0 { times - 1} else { times};
    return (r, times);
}


fn wrapping_sub_limit(x: u16, y: u16, limit: u16) -> (u16, u16) {
    let l = limit as u32;
    let times = y / limit;
    let r = ((x as u32 + l - (y as u32 % l)) % l) as u16;
    let times = if y%limit > x { times +1 } else { times };
    let times = if r%limit == 0 && times > 0 {times -1} else {times};
    return (r, times);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Direction {
    Left,
    Right,
}

impl Direction{
    fn from_byte(direction: u8) -> Direction {
        if direction == b'L'{
            return Direction::Left;
        }else if direction == b'R'{
            return Direction::Right;
        }else{
            panic!("Unknown direction: {}", direction);
        }
    }
}

impl Lock {
    pub(crate) fn new(file_name: &str) -> Lock {
        let path = env::current_dir();
        path.expect("Could not access current directory");
        let mut file = File::open(file_name).expect("Instantiating failed because:\nFile not found");
        file.seek(SeekFrom::Start(0)).unwrap();
        Lock{active_num: 50,
            null_counter: 0,
            null_passed_counter: 0,
            reader: BufReader::new(file), }
    }

    fn get_next_line(&mut self) -> String {
        let mut data: Vec<u8> = Vec::new();
        let n =self.reader.read_until(b'\n', &mut data).unwrap();
        if n == 0{
            return String::from("");
        }
         String::from_utf8(data).expect("Invalid UTF-8")
    }



    pub(crate) fn rotate(&mut self, direction: Direction, number_rotations: u16){
        println!("Rotating number: {:?}", number_rotations);
        if direction == Direction::Left {
            let ret  = wrapping_sub_limit(self.active_num as u16, number_rotations, LIMIT);
            self.active_num = ret.0 as u8;
            self.null_passed_counter += ret.1 as usize;
        }else{
            let ret = wrapping_add_limit(self.active_num as u16, number_rotations, LIMIT);
            self.active_num = ret.0 as u8;
            self.null_passed_counter += ret.1 as usize;
        }
        if self.active_num == 0{
            self.null_counter +=1;
        }
    }

    fn calculate_next_rotation(&mut self)-> bool{
        let active = self.get_next_line();
        if active == ""{
            return false;
        }
        let packet = Self::parse_line(active);
        self.rotate(packet.0, packet.1);
        true
    }

    fn parse_line(line: String) -> (Direction, u16) {
        if line.len() < 2 {
            panic!("Invalid line format (<2 chars)");
        }
        let dir_byte = line.as_bytes()[0];
        let num_rotation: u16 = line[1..].trim().parse().unwrap();

        (Direction::from_byte(dir_byte), num_rotation)
    }


    pub(crate) fn calculate_lock(&mut self){
        loop {
            if !self.calculate_next_rotation(){
                break;
            }
        }
    }
}