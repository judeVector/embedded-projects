#![no_std]
#![no_main]

use cyw43::aligned_bytes;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_rp::{
    bind_interrupts,
    dma::{Channel, InterruptHandler as DmaInterruptHandler},
    gpio::{Input, Level, Output, Pull},
    peripherals::{DMA_CH0, PIO0},
    pio::{InterruptHandler as PioInterruptHandler, Pio},
};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"dual-blink-toggle"),
    embassy_rp::binary_info::rp_program_description!(
        c"Toggle dual blinking on Pin 0 and Onboard LED using a button on GP15"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => PioInterruptHandler<PIO0>;
    DMA_IRQ_0 => DmaInterruptHandler<DMA_CH0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    let power = Output::new(p.PIN_23, Level::Low);
    let chip_select = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        chip_select,
        p.PIN_24,
        p.PIN_29,
        Channel::new(p.DMA_CH0, Irqs),
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, power, spi, fw, nvram).await;
    spawner.spawn(unwrap!(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let mut external_led = Output::new(p.PIN_0, Level::Low);
    let mut button = Input::new(p.PIN_15, Pull::Up);

    let mut is_blinking = false; // Dont start blinking immediately

    info!("System Ready. Press button on GP15 to start blinking.");

    loop {
        if is_blinking {
            // Onboard ON, external OFF
            control.gpio_set(0, true).await;
            external_led.set_low();
            info!("Onboard LED ON");

            // Wait 500ms OR until button pressed
            match select(
                Timer::after(Duration::from_millis(500)),
                button.wait_for_low(),
            )
            .await
            {
                Either::First(_) => {
                    // Timer finished normally, continue to next state
                }
                Either::Second(_) => {
                    // Button pressed during ON phase
                    Timer::after(Duration::from_millis(50)).await; // Debounce
                    if button.is_low() {
                        is_blinking = false;
                        control.gpio_set(0, false).await;
                        external_led.set_low();
                        info!("Blinking stopped!");
                        button.wait_for_high().await;
                        Timer::after(Duration::from_millis(50)).await;
                        continue;
                    }
                }
            }

            // Onboard OFF, external ON
            control.gpio_set(0, false).await;
            external_led.set_high();
            info!("External LED ON");

            // Wait 500ms OR until button pressed
            match select(
                Timer::after(Duration::from_millis(500)),
                button.wait_for_low(),
            )
            .await
            {
                Either::First(_) => {
                    // Timer finished normally, loop continues
                }
                Either::Second(_) => {
                    // Button pressed during OFF phase
                    Timer::after(Duration::from_millis(50)).await;
                    if button.is_low() {
                        is_blinking = false;
                        control.gpio_set(0, false).await;
                        external_led.set_low();
                        info!("Blinking stopped!");
                        button.wait_for_high().await;
                        Timer::after(Duration::from_millis(50)).await;
                        continue;
                    }
                }
            }
        } else {
            // Not blinking - wait for button press to start
            control.gpio_set(0, false).await;
            external_led.set_low();

            button.wait_for_low().await;
            Timer::after(Duration::from_millis(50)).await;

            if button.is_low() {
                is_blinking = true;
                info!("Blinking started!");
                button.wait_for_high().await;
                Timer::after(Duration::from_millis(50)).await;
            }
        }
    }
}
