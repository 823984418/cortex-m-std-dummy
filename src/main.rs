#![no_main]

mod sys;

use cortex_m_rt::entry;
use std::sync::atomic::{AtomicU32, Ordering};

#[entry]
fn main() -> ! {
    let x = AtomicU32::new(0);
    x.fetch_add(1, Ordering::AcqRel);
    loop {}
}
