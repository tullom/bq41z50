# BQ41Z50 Rust Device Driver

[![no-std](https://github.com/OpenDevicePartnership/bq41z50/actions/workflows/nostd.yml/badge.svg)](https://github.com/OpenDevicePartnership/bq41z50/actions/workflows/nostd.yml)
[![check](https://github.com/OpenDevicePartnership/bq41z50/actions/workflows/check.yml/badge.svg)](https://github.com/OpenDevicePartnership/bq41z50/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

An asynchronous, platform-agnostic Rust driver for the
[Texas Instruments BQ41Z50](https://www.ti.com/product/BQ41Z50) lithium-ion battery
pack manager and fuel gauge.

## Capabilities

- Async I2C communication using the
   [`embedded-hal-async`](https://docs.rs/embedded-hal-async) traits.
- `no_std` support on bare-metal targets (`target_os = "none"`).
- A high-level Smart Battery Specification (SBS) API through
   [`embedded-batteries-async`](https://docs.rs/embedded-batteries-async), including
   voltage, current, state of charge, capacity, and battery status.
- Typed register and ManufacturerAccess command access generated with
   [`device-driver`](https://docs.rs/device-driver).
- Temperature helpers returning distinct Celsius and Fahrenheit types.

## Installation

Add the driver from this repository, along with the traits used in the example,
to your application's dependencies:

```toml
[dependencies]
bq41z50 = { git = "https://github.com/OpenDevicePartnership/bq41z50" }
embedded-batteries-async = "0.3.4"
embedded-hal-async = "1.0.0"
```

No feature flags are required for the APIs shown below.

## Usage

Pass an initialized bus implementing `embedded_hal_async::i2c::I2c` to
`Bq41z50::new`. The driver uses the 7-bit Smart Battery address `0x0B`. Your
application supplies the platform HAL and async executor.

Bring `SmartBattery` into scope to use the SBS methods:

```rust,no_run
use bq41z50::{Bq41z50, Bq41z50Error, CentiCelsius};
use embedded_batteries_async::smart_battery::SmartBattery;
use embedded_hal_async::i2c::I2c;

async fn read_battery<BUS: I2c>(
      bus: BUS,
) -> Result<(u16, u8, CentiCelsius), Bq41z50Error<BUS::Error>> {
      let mut battery = Bq41z50::new(bus);

      let voltage_mv = battery.voltage().await?;
      let charge_percent = battery.relative_state_of_charge().await?;
      let temperature = battery.temperature_celsius().await?;

      Ok((voltage_mv, charge_percent, temperature))
}
```

Call `read_battery(bus).await` from your async application. It returns pack
voltage in millivolts, relative state of charge in percent, and temperature in
centi-degrees Celsius. Use `CentiCelsius::to_degrees()` to obtain degrees Celsius
as an `f64`. Errors are propagated as `Bq41z50Error`.

## Register Access

The public `device` field provides typed register and command access alongside
the SBS trait API. For example, `battery.device.serial_number().read_async().await`
reads the serial-number register, while
`battery.device.mac_chem_id().dispatch_out_async().await` reads the chemistry ID.

The [hardware example](examples/hello.rs) demonstrates both API levels. It uses
`pico-de-gallo-hal` and requires compatible I2C hardware connected to a BQ41Z50.

## Documentation

Build the API reference locally with `cargo doc --no-deps --open`. Consult the
[TI product documentation](https://www.ti.com/product/BQ41Z50) for device behavior,
register definitions, and electrical requirements.

## MSRV

The minimum supported Rust version is **1.94**.

## License

Licensed under the [MIT license](LICENSE).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) and the
[Code of Conduct](CODE_OF_CONDUCT.md) before submitting a contribution. Contributions
are accepted under the MIT license. To report a security vulnerability, follow the
[security policy](SECURITY.md).
