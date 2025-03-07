# esp-wifi

This is experimental and a work-in-progress! You are welcome to experiment with it and contribute but probably shouldn't use this for something real yet.

Wi-Fi/BTLE coexistence is implemented but currently only works (to some extent) on ESP32-C3 and ESP32-S3. In general COEX shouldn't be used currently.

Minimum supported Rust compiler version: 1.65.0.0

This uses the WiFi drivers from https://github.com/esp-rs/esp-wireless-drivers-3rdparty

## Version used

v5.1-dev-2658-g0025915dc4 commit 0025915dc489a9d45f99aed74920346f8ac4ec09

https://github.com/esp-rs/esp-wireless-drivers-3rdparty/ (commit f4caebff200e8f6f51b0a11d2b69ca56c76bb1c9)

## Current support

If a cell contains am em dash (&mdash;) this means that the particular feature is not present for a chip. A check mark (✓) means that some driver implementation exists. An empty cell means that the feature is present in the chip but not implemented yet.

|          | [Wifi](https://github.com/esp-rs/esp-wifi/issues/94) | [BLE](https://github.com/esp-rs/esp-wifi/issues/93) | [Coex](https://github.com/esp-rs/esp-wifi/issues/92) | ESP-NOW |
| :------: | :--------------------------------------------------: | :-------------------------------------------------: | :--------------------------------------------------: | :-----: |
|  ESP32   |                          ✓                           |                          ✓                          |                                                      |    ✓    |
| ESP32-S2 |                          ✓                           |                       &mdash;                       |                       &mdash;                        |    ✓    |
| ESP32-S3 |                          ✓                           |                          ✓                          |                          ✓                           |    ✓    |
| ESP32-C3 |                          ✓                           |                          ✓                          |                          ✓                           |    ✓    |
| ESP32-C2 |                          ✓                           |                          ✓                          |                                                      |    ✓    |

## Examples

To build these ensure you are in the `esp-wifi` directory as othewise the `config.toml` will not apply

### dhcp
#### Run .\private.ps1 to set this (be sure to .gitignore this script)
- set SSID and PASSWORD via env variables
- gets an ip address via DHCP
- performs an HTTP get request to some "random" server

|   Chip   | Command |
| :------: | ------- |
|  ESP32   | `cargo +esp run --example dhcp --release --target xtensa-esp32-none-elf --features "esp32,embedded-svc,wifi"` |
| ESP32-C2 | `cargo +nightly run --example dhcp --release --target riscv32imc-unknown-none-elf --features "esp32c2,embedded-svc,wifi"` |
| ESP32-C3 | `cargo +nightly run --example dhcp --release --target riscv32imc-unknown-none-elf --features "esp32c3,embedded-svc,wifi"` |
| ESP32-S2 | `cargo +esp run --example dhcp --release --target xtensa-esp32s2-none-elf --features "esp32s2,embedded-svc,wifi"` |
| ESP32-S3 | `cargo +esp run --example dhcp --release --target xtensa-esp32s3-none-elf --features "esp32s3,embedded-svc,wifi"` |

### static_ip

- set SSID and PASSWORD env variable
- set STATIC_IP and GATEWAY_IP env variable (e.g. "192.168.2.191" / "192.168.2.1")
- might be necessary to configure your WiFi access point accordingly
- uses the given static IP
- responds with some HTML content when connecting to port 8080

|   Chip   | Command                                                                                                                        |
| :------: | ------------------------------------------------------------------------------------------------------------------------------ |
|  ESP32   | `cargo +esp run --example static_ip --release --target xtensa-esp32-none-elf --features "esp32,embedded-svc,wifi"`             |
| ESP32-C2 | `cargo +nightly run --example static_ip --release --target riscv32imc-unknown-none-elf --features "esp32c2,embedded-svc,wifi"` |
| ESP32-C3 | `cargo +nightly run --example static_ip --release --target riscv32imc-unknown-none-elf --features "esp32c3,embedded-svc,wifi"` |
| ESP32-S2 | `cargo +esp run --example static_ip --release --target xtensa-esp32s2-none-elf --features "esp32s2,embedded-svc,wifi"`         |
| ESP32-S3 | `cargo +esp run --example static_ip --release --target xtensa-esp32s3-none-elf --features "esp32s3,embedded-svc,wifi"`         |

### ble

- starts Bluetooth advertising
- offers one service with three characteristics (one is read/write, one is write only, one is read/write/notify)
- pressing the boot-button on a dev-board will send a notification if it is subscribed
- this uses a toy level BLE stack - might not work with every BLE central device (tested with Android and ~~Windows Bluetooth LE Explorer~~ --> Ugh.)  Try [iPhone app BLE Scanner 4.0](https://apps.apple.com/us/app/ble-scanner-4-0/id1221763603)

|   Chip   | Command                                                                                                                                    |
| :------: | ------------------------------------------------------------------------------------------------------------------------------------------ |
|  ESP32   | `cargo +esp run --example ble --release --target xtensa-esp32-none-elf --features "esp32,ble"`                                             |
| ESP32-C2 | `CARGO_PROFILE_RELEASE_LTO=false cargo +nightly run --example ble --release --target riscv32imc-unknown-none-elf --features "esp32c2,ble"` |
| ESP32-C3 | `cargo +nightly run --example ble --release --target riscv32imc-unknown-none-elf --features "esp32c3,ble"`                                 |
| ESP32-S3 | `cargo +esp run --example ble --release --target xtensa-esp32s3-none-elf --features "esp32s3,ble"`                                         |

**NOTE:** ESP32-S2 doesn't support bluetooth

### coex

- set SSID and PASSWORD env variable
- gets an ip address via DHCP
- performs an HTTP get request to some "random" server
- does BLE advertising
- coex support is still somewhat flaky

|   Chip   | Command                                                                                                                       |
| :------: | ----------------------------------------------------------------------------------------------------------------------------- |
| ESP32-C3 | `cargo +nightly run --example coex --release --target riscv32imc-unknown-none-elf --features "esp32c3,embedded-svc,wifi,ble"` |
| ESP32-S3 | `cargo +esp run --example coex --release --target xtensa-esp32s3-none-elf --features "esp32s3,embedded-svc,wifi,ble"`         |

**NOTE:** Not currently available for the ESP32, ESP32-C2, or ESP32-S2

### esp_now

- broadcasts, receives and sends messages via esp-now (See tutorial [here](
https://dronebotworkshop.com/esp-now/ "ESP NOW – Peer to Peer ESP32 Communications").)

|   Chip   | Command                                                                                                                                            |
| :------: | -------------------------------------------------------------------------------------------------------------------------------------------------- |
|  ESP32   | `cargo +esp run --example esp_now --release --target xtensa-esp32-none-elf --features "esp32,esp-now"`                                             |
| ESP32-C2 | `CARGO_PROFILE_RELEASE_LTO=false cargo +nightly run --example esp_now --release --target riscv32imc-unknown-none-elf --features "esp32c2,esp-now"` |
| ESP32-C3 | `cargo +nightly run --example esp_now --release --target riscv32imc-unknown-none-elf --features "esp32c3,esp-now"`                                 |
| ESP32-S2 | `cargo +esp run --example esp_now --release --target xtensa-esp32s2-none-elf --features "esp32s2,esp-now"`                                         |
| ESP32-S3 | `cargo +esp run --example esp_now --release --target xtensa-esp32s3-none-elf --features "esp32s3,esp-now"`                                         |

### embassy_esp_now

- broadcasts, receives and sends messages via esp-now in an async way

|   Chip   | Command                                                                                                                                                                  |
| :------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|  ESP32   | `cargo +esp run --example embassy_esp_now --release --target xtensa-esp32-none-elf --features "esp32,esp32-async,esp-now"`                                               |
| ESP32-C2 | `CARGO_PROFILE_RELEASE_LTO=false cargo +nightly run --example embassy_esp_now --release --target riscv32imc-unknown-none-elf --features "esp32c2,esp32c2-async,esp-now"` |
| ESP32-C3 | `cargo +nightly run --example embassy_esp_now --release --target riscv32imc-unknown-none-elf --features "esp32c3,esp32c3-async,esp-now"`                                 |
| ESP32-S2 | `cargo +esp run --example embassy_esp_now --release --target xtensa-esp32s2-none-elf --features "esp32s2,esp32s2-async,esp-now"`                                         |
| ESP32-S3 | `cargo +esp run --example embassy_esp_now --release --target xtensa-esp32s3-none-elf --features "esp32s3,esp32s3-async,esp-now"`        

### embassy_dhcp

- Read and Write to sockets over WiFi asyncronously using embassy-executor.

|   Chip   | Command                                                                                                                                                                  |
| :------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
|  ESP32   | `cargo +esp run --example embassy_dhcp --release --target xtensa-esp32-none-elf --features "esp32,esp32-async,embedded-svc,wifi,embassy-net"`                                               |
| ESP32-C2 | `CARGO_PROFILE_RELEASE_LTO=false cargo +nightly run --example embassy_dhcp --release --target riscv32imc-unknown-none-elf --features "esp32c2,esp32c2-async,embedded-svc,wifi,embassy-net"` |
| ESP32-C3 | `cargo +nightly run --example embassy_dhcp --release --target riscv32imc-unknown-none-elf --features "esp32c3,esp32c3-async,embedded-svc,wifi,embassy-net"`                                 |
| ESP32-S2 | `cargo +esp run --example embassy_dhcp --release --target xtensa-esp32s2-none-elf --features "esp32s2,esp32s2-async,embedded-svc,wifi,embassy-net"`                                         |
| ESP32-S3 | `cargo +esp run --example embassy_dhcp --release --target xtensa-esp32s3-none-elf --features "esp32s3,esp32s3-async,embedded-svc,wifi,embassy-net"`                                    |

## Features

| Feature      | Meaning                                                                                             |
| ------------ | --------------------------------------------------------------------------------------------------- |
| wifi-logs    | logs the WiFi logs from the driver at log level info                                                |
| dump-packets | dumps some packet info at log level info                                                            |
| utils        | Provide utilities for smoltcp initialization, this is a default feature                             |
| embedded-svc | Provides a (very limited) implementation of the `embedded-svc` WiFi trait, includes `utils` feature |
| ble          | Enable BLE support                                                                                  |
| wifi         | Enable WiFi support                                                                                 |
| esp-now      | Enable esp-now support                                                                              |

## Important

## Optimization Level

It is necessary to build with optimization level 2 or 3 since otherwise it might not even be able to connect or advertise.

To make it work also for your debug builds add this to your `Cargo.toml`

```toml
[profile.dev.package.esp-wifi]
opt-level = 3
```

## LTO

Link time optimization is not yet recommended for use, please ensure `lto = "off"` is in your `Cargo.toml` for both release and debug profiles.

## Using Serial-JTAG

On ESP32-C3 / ESP32-S3 when using Serial-JTAG you have to activate the feature `phy-enable-usb`.

Don't use this feature if your are _not_ using Serial-JTAG since it might reduce WiFi performance.

## What works?

- scanning for WiFi access points
- connect to WiFi access point
- providing an HCI interface

## Notes on ESP32-C3 support

- uses SYSTIMER as the main timer
- doesn't work in direct-boot mode

## Notes on ESP32 / ESP32-S3 support

This is even more experimental than support for ESP32-C3.

- The WiFi logs only print the format string - not the actual values.
- Also there might be some packet loss and a bit worse performance than on ESP32-C3 currently.
- The code runs on a single core and might currently not be multi-core safe!

On ESP32 / ESP32-S3 currently TIMG1/TIMER0 is used as the main timer so you can't use it for anything else.
Additionally it uses CCOMPARE0 - so don't touch that, too.

## opt-level for Xtensa targets

Currently your mileage might vary a lot for different opt-levels on Xtensa targets!
If something doesn't work as expected try a different opt-level.

## Directory Structure

- `src/timer-espXXX.rs`: systimer code used for timing and task switching
- `src/preemt/`: a bare minimum RISCV and Xtensa round-robin task scheduler
- `src/compat/`: code needed to emulate enough of an (RT)OS to use the driver
  - `common.rs`: basics like semaphores and recursive mutexes
  - `timer_compat.rs`: code to emulate timer related functionality
- `examples/*.rs`: examples

## Missing / To be done

- lots of refactoring
- make CoEx work on ESP32 (it kind of works when commenting out setting the country in wifi_start, probably some mis-compilation since it then crashes in a totally different code path)
- maybe SoftAP

## Using in your own binary crate

For now this is not available on _crates.io_. Until then you need to specify a git dependency.
You might want to pin the dependency to a specific commit since this things might change a lot during development.

Make sure to include the rom functions for your target like this

```toml
rustflags = [
    "-C", "link-arg=-Tlinkall.x",
    "-C", "link-arg=-Trom_functions.x",
]
```

in your `.cargo/config.toml` - otherwise you will get linker errors complaining about missing symbols.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
