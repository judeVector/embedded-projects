#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_0, Level::Low);

    let button = Input::new(p.PIN_15, Pull::None);
    let mut led_state = false;

    loop {
        if button.is_low() {
            led_state = !led_state;
            if led_state {
                led.set_high();
            }
        } else {
            if led_state {
                led.set_low();
            }
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}
