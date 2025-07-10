#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use libtock::{console::ConsoleAsync, leds::Leds};

#[embassy_executor::main(stack_size=0x3000)]
async fn main(_spawner: Spawner) {
    loop {
        ConsoleAsync::write("Hello from Tock\n".as_bytes()).await.unwrap();
        Leds::toggle(0).unwrap();
        Timer::after_secs(2).await;
    }
}
