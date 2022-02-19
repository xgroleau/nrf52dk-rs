#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::config::Config;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive, Pin as _};
use embassy_nrf::interrupt::Priority;
use embassy_nrf::Peripherals;
use nrf_softdevice as _;
use nrf_softdevice::Softdevice;
use panic_probe as _;

mod bluetooth;

fn embassy_config() -> Config {
    let mut config = Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    config
}

#[embassy::task]
async fn blinky(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(300)).await;
        led.set_low();
        Timer::after(Duration::from_millis(300)).await;
    }
}

#[embassy::main(config = "embassy_config()")]
async fn main(spawner: Spawner, p: Peripherals) {
    let config = bluetooth::softdevice_config();
    let sd = Softdevice::enable(&config);

    defmt::unwrap!(spawner.spawn(blinky(p.P0_13.degrade())));
    defmt::unwrap!(spawner.spawn(bluetooth::softdevice_task(sd)));
    defmt::unwrap!(spawner.spawn(bluetooth::bluetooth_task(
        sd,
        p.P0_11.degrade(),
        p.P0_12.degrade()
    )));
}
