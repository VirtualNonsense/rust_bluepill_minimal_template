#![no_std]
#![no_main]

//
use cortex_m_rt::entry;
// supplies the panic handler
use panic_halt as _;

// supplies rust-lld
use stm32f1xx_hal; // use this line when actually programming

#[entry]
fn main() -> ! {
    loop {}
}