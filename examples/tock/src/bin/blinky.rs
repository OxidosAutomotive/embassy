#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use libtock::{console::{ConsoleAsync, ConsoleBufWriter}, leds::Leds};
use core::{fmt::Write, pin::pin};

#[embassy_executor::main(stack_size=0x3000)]
async fn main(_spawner: Spawner) {
    loop {
        let mut buffer: ConsoleBufWriter<32> = ConsoleBufWriter::new();
        writeln!(buffer, "Hello from Tock!").unwrap();
        ConsoleAsync::write(&mut pin!(buffer.into_allow_ro_buffer())).await.unwrap();
        Leds::toggle(0).unwrap();
        Timer::after_secs(2).await;
    }
}
