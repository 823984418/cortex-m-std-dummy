#![no_main]

mod sys;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
