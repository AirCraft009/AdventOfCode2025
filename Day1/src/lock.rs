use std::cmp::PartialEq;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

const LIMIT: u8 = 100;
pub(crate) struct Lock {
    active_num: u8,
    end: u8,
    pub(crate) null_counter: u8,
    reader: BufReader<File>,
}

fn wrapping_add_limit(x: u8, y: u8, limit: u8) -> u8 {
    let range = limit + 1;
    (x + y) % range
}


fn wrapping_sub_limit(x: u8, y: u8, limit: u8) -> u8 {
    let range = limit + 1;
    (x + range - y) % range
}


enum Direction {
    Left,
    Right,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
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
        println!("The current directory is {}", path.expect("REASON").display());
        let mut file = File::open(file_name).expect("Instantiating failed because:\nFile not found");
        file.seek(SeekFrom::Start(0)).unwrap();
        Lock{active_num: 0,
            end: LIMIT,
            null_counter: 0,
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



    fn rotate(&mut self, direction: Direction, number_rotations: u8){
        if direction == Direction::Left {
            self.active_num = wrapping_sub_limit(self.active_num, number_rotations, self.end);
        }else{
            self.active_num = wrapping_add_limit(self.active_num, number_rotations, self.end);
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

    fn parse_line(line: String) -> (Direction, u8) {
        if line.len() < 2 {
            panic!("Invalid line format (<2 chars)");
        }
        println!("{}", line);
        let dir_byte = line.as_bytes()[0];
        let num_rotation: u8 = line[1..].trim().parse().unwrap();

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