use defmt;
use nrf_softdevice::ble::{gatt_server, peripheral};
use nrf_softdevice::{raw, Config, Softdevice};

mod bluetooth {
    #[embassy::task]
    async fn softdevice_task(sd: &'static Softdevice) {
        sd.run().await;
    }

    #[nrf_softdevice::gatt_service(uuid = "9e7312e0-2354-11eb-9f10-fbc30a62cf38")]
    struct FooService {
        #[characteristic(uuid = "9e7312e0-2354-11eb-9f10-fbc30a63cf38", read, write, notify)]
        foo: u16,
    }

    #[nrf_softdevice::gatt_server]
    struct Server {
        foo: FooService,
    }

    async fn run_bluetooth(sd: &'static Softdevice, server: &Server) {
        loop {
            config = peripheral::Config::default();
            adb = peripheral::ConnectableAdvertisement::ScannableUndirected {
                adv_data,
                scan_data,
            };

            let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);

            defmt::info!("Advertising done!");

            let result = gatt_server::run(&conn, server, |e| match e {
                ServerEvent::Foo(FooServiceEvent::FooWrite(val)) => {
                    defmt::info!("Wrote foo level: {}", val);
                    if let Err(e) = server.foo.foo_notify(&conn, val + 1) {
                        defmt::info!("Sent notification error {:?}", e);
                    }
                }

                ServerEvent::Foo(FooServiceEvent::FooCccdWrite { notifications }) => {
                    defmt::info!("Foo notifications: {}", notifications);
                }
            })
            .await;

            if let Err(e) = res {
                defmt::info!("Gatt server exited with error: {:?}", e);
            }
        }
    }

    fn softdevice_config() -> Config {
        let config = Config {
            clock: Some(raw::nrf_clock_lf_cfg_t {
                source: raw::NRF_CLOCK_LF_SRC_RC as u8,
                rc_ctiv: 4,
                rc_temp_ctiv: 2,
                accuracy: 7,
            }),
            conn_gap: Some(raw::ble_gap_conn_cfg_t {
                conn_count: 6,
                event_length: 24,
            }),
            conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
            gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
                attr_tab_size: 32768,
            }),
            gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
                adv_set_count: 1,
                periph_role_count: 3,
                central_role_count: 3,
                central_sec_count: 0,
                _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
            }),
            gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
                p_value: b"HelloRust" as *const u8 as _,
                current_len: 9,
                max_len: 9,
                write_perm: unsafe { mem::zeroed() },
                _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                    raw::BLE_GATTS_VLOC_STACK as u8,
                ),
            }),
            ..Default::default()
        };
    }

    #[embassy::task]
    async fn bluetooth_task(sd: &'static Softdevice, button1: AnyPin, button2: AnyPin) {
        let server: Server = unwrap!(gatt_server::register(sd));

        defmt::info!("Bluetooth is OFF");
        defmt::info!("Press nrf52840-dk button 1 to enable, button 2 to disable");

        let button1 = Input::new(button1, Pull::Up);
        let button2 = Input::new(button2, Pull::Up);
        pin_mut!(button1);
        pin_mut!(button2);
        loop {
            button1.as_mut().wait_for_low().await;
            info!("Bluetooth ON!");

            // Create a future that will run the bluetooth loop.
            // Note the lack of `.await`! This creates the future but doesn't poll it yet.
            let bluetooth_fut = run_bluetooth(sd, &server);

            // Create a future that will resolve when the OFF button is pressed.
            let off_fut = async {
                button2.as_mut().wait_for_low().await;
                info!("Bluetooth OFF!");
            };

            pin_mut!(bluetooth_fut);
            pin_mut!(off_fut);

            // Select the two futures.
            //
            // select() returns when one of the two futures returns. The other future is dropped before completing.
            //
            // Since the bluetooth future never finishes, this can only happen when the Off button is pressed.
            // This will cause the bluetooth future to be dropped.
            //
            // If it was advertising, the nested `peripheral::advertise_connectable` future will be dropped, which will cause
            // the softdevice to stop advertising.
            // If it was connected, it will drop everything including the `Connection` instance, which
            // will tell the softdevice to disconnect it.
            //
            // This demonstrates the awesome power of Rust's async-await combined with nrf-softdevice's async wrappers.
            // It's super easy to cancel a complex tree of operations: just drop its future!
            futures::future::select(bluetooth_fut, off_fut).await;
        }
    }
}
