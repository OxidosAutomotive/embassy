#![cfg_attr(not(any(feature = "std", feature = "wasm", test)), no_std)]
#![allow(async_fn_in_trait)]
#![doc = include_str!("../README.md")]
#![allow(clippy::new_without_default)]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]

//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

mod delay;
#[cfg_attr(feature = "dynamic-tick-rate", path = "duration_dynamic.rs")]
mod duration;
#[cfg_attr(feature = "dynamic-tick-rate", path = "instant_dynamic.rs")]
mod instant;
mod timer;

#[cfg(feature = "mock-driver")]
mod driver_mock;

#[cfg(feature = "mock-driver")]
pub use driver_mock::MockDriver;

#[cfg(feature = "std")]
mod driver_std;
#[cfg(feature = "wasm")]
mod driver_wasm;

pub use delay::{block_for, Delay};
pub use duration::Duration;
#[cfg(not(feature = "dynamic-tick-rate"))]
pub use embassy_time_driver::TICK_HZ;
#[cfg(feature = "dynamic-tick-rate")]
pub use embassy_time_driver::frequency;
pub use instant::Instant;
pub use timer::{with_deadline, with_timeout, Ticker, TimeoutError, Timer, WithTimeout};

const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(not(feature = "dynamic-tick-rate"))]
pub(crate) const GCD_1K: u64 = gcd(TICK_HZ, 1_000);
#[cfg(not(feature = "dynamic-tick-rate"))]
pub(crate) const GCD_1M: u64 = gcd(TICK_HZ, 1_000_000);
#[cfg(not(feature = "dynamic-tick-rate"))]
pub(crate) const GCD_1G: u64 = gcd(TICK_HZ, 1_000_000_000);

#[cfg(feature = "dynamic-tick-rate")]
#[inline(always)]
pub(crate) fn gcd_1k() -> u64 {
    return gcd(frequency(), 1_000);
}

#[cfg(feature = "dynamic-tick-rate")]
#[inline(always)]
pub(crate) fn gcd_1m() -> u64 {
    return gcd(frequency(), 1_000_000);
}

#[cfg(feature = "dynamic-tick-rate")]
#[inline(always)]
pub(crate) fn gcd_1g() -> u64 {
    return gcd(frequency(), 1_000_000_000);
}

#[cfg(feature = "defmt-timestamp-uptime-s")]
defmt::timestamp! {"{=u64}", Instant::now().as_secs() }

#[cfg(feature = "defmt-timestamp-uptime-ms")]
defmt::timestamp! {"{=u64:ms}", Instant::now().as_millis() }

#[cfg(any(feature = "defmt-timestamp-uptime", feature = "defmt-timestamp-uptime-us"))]
defmt::timestamp! {"{=u64:us}", Instant::now().as_micros() }

#[cfg(feature = "defmt-timestamp-uptime-ts")]
defmt::timestamp! {"{=u64:ts}", Instant::now().as_secs() }

#[cfg(feature = "defmt-timestamp-uptime-tms")]
defmt::timestamp! {"{=u64:tms}", Instant::now().as_millis() }

#[cfg(feature = "defmt-timestamp-uptime-tus")]
defmt::timestamp! {"{=u64:tus}", Instant::now().as_micros() }
