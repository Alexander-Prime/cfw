#![no_std]
#![no_main]
#![feature(never_type)]

mod controller;
mod driver;
mod logging;

use controller::Controller;
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    logging::init().unwrap();
    let c_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(c_peripherals.SYST);
    systick.delay(500);

    let controller = Controller::new();

    for frame in controller {
        log::info!("{}", frame);
    }

    log::info!("Iterator ended, halting");

    loop {}
}
