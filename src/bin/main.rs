#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cortex_m_rt::entry;
use defmt;
use defmt_rtt as _;
use embassy::executor::Executor;
use embassy::time::{Duration, Timer};
use embassy::util::Forever;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::Peripherals;
use panic_probe as _;

static EXECUTOR: Forever<Executor> = Forever::new();

#[embassy::task]
async fn blink(p: Peripherals) {
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

#[entry]
fn main() -> ! {
    defmt::info!("Hello World!");

    let mut config = embassy_nrf::config::Config::default();
    let p = embassy_nrf::init(config);

    let executor = EXECUTOR.put(Executor::new());
    executor.run(|spawner| {
        defmt::unwrap!(spawner.spawn(blink(p)));
    });
}
