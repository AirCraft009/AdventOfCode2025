
use crate::lock::Lock;

mod lock;

fn main() {
    let mut lock = Lock::new("input.txt");
    lock.calculate_lock();
    println!("{}", lock.null_counter)
}



