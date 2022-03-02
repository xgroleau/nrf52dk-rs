# nRF52DK-rs

An application exploring rust on the nrf52840-dk with [probe-run](https://crates.io/crates/probe-run), [defmt](https://github.com/knurling-rs/defmt), [flip-link](https://github.com/knurling-rs/flip-link), [embassy](https://github.com/embassy-rs/embassy)


## Setup

### Nix 
If you use [`nix`](https://nixos.org/), you can simply use `nix develop` and all the dependencies will be downloaded.

### Cargo
Install flip-link and probe-run and probe-rs-cli.
```console
$ cargo install flip-link probe-run probe-rs-cli
```

## Setup

### Softdevice
You need to flash the softdevice when the device is erased.
``` sh
probe-rs-cli download --format hex softdevices/s140_nrf52_7.2.0_softdevice.hex  --chip nRF52840_xxAA --chip-erase
```


## Running tests

The template comes configured for running unit tests and integration tests on the target.

Unit tests reside in the library crate and can test private API; the initial set of unit tests are in `src/lib.rs`.
`cargo test --lib` will run those unit tests.

``` console
$ cargo test --lib
(1/1) running `it_works`...
└─ app::unit_tests::__defmt_test_entry @ src/lib.rs:33
all tests passed!
└─ app::unit_tests::__defmt_test_entry @ src/lib.rs:28
```

Integration tests reside in the `tests` directory; the initial set of integration tests are in `tests/integration.rs`.
`cargo test --test integration` will run those integration tests.
Note that the argument of the `--test` flag must match the name of the test file in the `tests` directory.

``` console
$ cargo test --test integration
(1/1) running `it_works`...
└─ integration::tests::__defmt_test_entry @ tests/integration.rs:13
all tests passed!
└─ integration::tests::__defmt_test_entry @ tests/integration.rs:8
```

Note that to add a new test file to the `tests` directory you also need to add a new `[[test]]` section to `Cargo.toml`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
