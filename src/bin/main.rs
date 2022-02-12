#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::Peripherals;
use panic_probe as _;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        defmt::info!("Seting high");
        Timer::after(Duration::from_millis(300)).await;
        led.set_low();
        defmt::info!("Seting low");
        Timer::after(Duration::from_millis(300)).await;
    }
}
