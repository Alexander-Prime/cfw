#![no_std]
#![no_main]
#![feature(never_type)]

mod controller;
mod logging;

use bsp::hal::ccm::spi::{ClockSelect, PrescalarSelect};
use controller::Controller;
use lsm6ds33::Lsm6ds33;
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    logging::init().unwrap();
    let c_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(c_peripherals.SYST);
    systick.delay(500);

    let mut peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::t41::into_pins(peripherals.iomuxc);

    // Get 4 SPI module builders
    // Not sure why we take the last one instead of, say, the first
    // Examples do it this way, so we do too for now
    let (_, _, _, spi4_builder) = peripherals.spi.clock(
        &mut peripherals.ccm.handle,
        ClockSelect::Pll2,
        PrescalarSelect::LPSPI_PODF_7,
    );

    // Create the LSM6DS33 driver
    let imu = {
        let mut cs = bsp::hal::gpio::GPIO::new(pins.p10).output();
        cs.set();
        let mut sixaxis_spi = spi4_builder.build(pins.p11, pins.p12, pins.p13);
        sixaxis_spi.set_mode(embedded_hal::spi::MODE_3).unwrap();
        systick.delay(1);
        Lsm6ds33::try_new(sixaxis_spi, cs)
            .unwrap_or_else(|_| panic!("Failed to create LSM6DS33 driver"))
    };

    let controller = Controller::new(imu);

    for frame in controller {
        log::info!("{}", frame);
    }

    log::info!("Iterator ended, halting");

    panic!()
}
