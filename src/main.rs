#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nrf52833_hal::pac::{self, TIMER0};
use nrf52833_hal::gpio::Level;
use embedded_hal::digital::OutputPin;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

const PIXELS: [(usize, usize); 16] = [
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4),
    (4, 3), (4, 2), (4, 1), (4, 0), (3, 0), (2, 0), (1, 0)
];

#[inline(never)]
fn delay(timer: &TIMER0, ms: u32) {
    unsafe {
        timer.tasks_clear.write(|w| w.bits(1));        // Clear the timer
        timer.tasks_start.write(|w| w.bits(1));        // Start the timer
        timer.cc[0].write(|w| w.bits(ms * 1_000));     // Set the compare register (1ms tick)
        
        while timer.events_compare[0].read().bits() == 0 {}  // Wait for the compare event
        
        timer.events_compare[0].write(|w| w.bits(0));   // Clear the compare event
        timer.tasks_stop.write(|w| w.bits(1));          // Stop the timer
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting...");

    let pac = pac::Peripherals::take().unwrap();
    let port0 = nrf52833_hal::gpio::p0::Parts::new(pac.P0);
    let port1 = nrf52833_hal::gpio::p1::Parts::new(pac.P1);

    // Configure LEDs as push-pull outputs
    let mut leds = [
        [port0.p0_03.into_push_pull_output(Level::Low).degrade(), port0.p0_04.into_push_pull_output(Level::Low).degrade(), port0.p0_28.into_push_pull_output(Level::Low).degrade(), port0.p0_29.into_push_pull_output(Level::Low).degrade(), port0.p0_31.into_push_pull_output(Level::Low).degrade()],
        [port1.p1_01.into_push_pull_output(Level::Low).degrade(), port0.p0_11.into_push_pull_output(Level::Low).degrade(), port0.p0_10.into_push_pull_output(Level::Low).degrade(), port0.p0_09.into_push_pull_output(Level::Low).degrade(), port0.p0_30.into_push_pull_output(Level::Low).degrade()],
        [port0.p0_12.into_push_pull_output(Level::Low).degrade(), port1.p1_02.into_push_pull_output(Level::Low).degrade(), port1.p1_08.into_push_pull_output(Level::Low).degrade(), port0.p0_05.into_push_pull_output(Level::Low).degrade(), port0.p0_02.into_push_pull_output(Level::Low).degrade()],
        [port1.p1_10.into_push_pull_output(Level::Low).degrade(), port1.p1_09.into_push_pull_output(Level::Low).degrade(), port0.p0_26.into_push_pull_output(Level::Low).degrade(), port0.p0_20.into_push_pull_output(Level::Low).degrade(), port0.p0_19.into_push_pull_output(Level::Low).degrade()],
        [port0.p0_13.into_push_pull_output(Level::Low).degrade(), port0.p0_14.into_push_pull_output(Level::Low).degrade(), port0.p0_15.into_push_pull_output(Level::Low).degrade(), port0.p0_16.into_push_pull_output(Level::Low).degrade(), port0.p0_08.into_push_pull_output(Level::Low).degrade()]
    ];

    let timer = &pac.TIMER0;
    
    // Configure TIMER0
    timer.mode.write(|w| w.mode().timer());          // Set timer mode
    timer.bitmode.write(|w| w.bitmode()._32bit());   // Set 32-bit mode
    timer.prescaler.write(|w| unsafe { w.bits(4) }); // Set prescaler to 16 (2^4 = 16, for 1 MHz timer clock)

    let mut last_led = (0, 0);

    loop {
        for &current_led in PIXELS.iter() {
            leds[last_led.0][last_led.1].set_low().unwrap();
            leds[current_led.0][current_led.1].set_high().unwrap();
            delay(timer, 50);
            last_led = current_led;
        }
    }
}
