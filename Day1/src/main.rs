use crate::lock::Lock;
use crate::lock::Direction::{Left, Right};

mod lock;

fn main() {
    let mut lock = Lock::new("input.txt");

    lock.calculate_lock();
    println!("on zero {}, clicked zero {}, password {}", lock.null_counter, lock.null_passed_counter, lock.null_passed_counter + lock.null_counter);

}



