use nrf_softdevice::ble::{gatt_server, peripheral};
use nrf_softdevice::{raw, Softdevice};

#[embassy::task]
async fn softdevice_task(sd: &'static Softdevice) {
    sd.run().await;
}
