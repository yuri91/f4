//! User LEDs

use stm32f40x::{GPIOD, RCC};

/// All the user LEDs
pub static LEDS: [Led; 4] = [
    Led { i: 12 }, // LD4, Green
    Led { i: 13 }, // LD3, Orange
    Led { i: 14 }, // LD5, Red
    Led { i: 15 }, // LD6, Blue
];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOD.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOD.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpiod: &GPIOD, rcc: &RCC) {
    // Power up peripherals
    rcc.ahb1enr.modify(|_, w| w.gpioden().enabled());

    // Configure pins 8-15 as outputs
    gpiod.moder.modify(|_, w| {
        w.moder12()
            .output()
            .moder13()
            .output()
            .moder14()
            .output()
            .moder15()
            .output()
    });
}

/// LED4, Orange
pub fn orange() -> &'static Led {
    &LEDS[0]
}
/// LED3, Green
pub fn green() -> &'static Led {
    &LEDS[1]
}
/// LED5, Red
pub fn red() -> &'static Led {
    &LEDS[2]
}
/// LED6, Blue
pub fn blue() -> &'static Led {
    &LEDS[3]
}
