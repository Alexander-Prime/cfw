#![no_std]
#![no_main]
#![feature(never_type)]

mod driver;
mod logging;

use bsp::hal::ccm::spi::{ClockSelect, PrescalarSelect};
use bsp::hal::gpio::GPIO;
use driver::glidepoint::Tm035035;
use firmware::engine::Engine;
use teensy4_bsp as bsp;
use teensy4_panic as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut peripherals = bsp::Peripherals::take().unwrap();
    let pins = bsp::t41::into_pins(peripherals.iomuxc);

    logging::init().unwrap();

    // Get 4 SPI module builders
    // Not sure why we take the last one instead of, say, the first
    // Examples do it this way, so we do too for now
    let (_, _, _, spi4_builder) = peripherals.spi.clock(
        &mut peripherals.ccm.handle,
        ClockSelect::Pll2,
        PrescalarSelect::LPSPI_PODF_7,
    );

    // Create the GlidePoint TM035035 driver
    // Will want to extend this at some point to allow multiple drivers on the same SPI bus via chip
    // select pins
    let (x, y, z) = {
        let mut touchpad_spi = spi4_builder.build(pins.p11, pins.p12, pins.p13);
        touchpad_spi.set_mode(embedded_hal::spi::MODE_1).unwrap();
        touchpad_spi.enable_chip_select_0(pins.p10);
        let touchpad_data_ready = GPIO::new(pins.p15);
        Tm035035::try_new(touchpad_spi, touchpad_data_ready)
            .unwrap_or_else(|_| panic!("Failed to create Tm035035 driver"))
    }
    .primitives();

    // Start the engine
    Engine::new().with_watcher(|| log::info!("loop")).start();
}
