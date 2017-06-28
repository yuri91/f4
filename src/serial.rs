//! Serial interface
//!
//! - TX - PA2
//! - RX - PA3

use core::ptr;

use cast::{u16, u8};
use stm32f40x::{GPIOA, RCC, USART2};

use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// Serial interface
///
/// # Interrupts
///
/// - `Usart1Exti25` - RXNE (RX buffer not empty)
#[derive(Clone, Copy)]
pub struct Serial<'a>(pub &'a USART2);

impl<'a> Serial<'a> {
    /// Initializes the serial interface with a baud rate of `baut_rate` bits
    /// per second
    pub fn init(self, gpioa: &GPIOA, rcc: &RCC, baud_rate: u32) {
        let usart2 = self.0;

        // Power up the peripherals
        rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
        rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());

        // Configure PA2 as TX and PA3 as RX
        gpioa.afrl.modify(|_, w| unsafe {
            w.afrl2().bits(7).afrl3().bits(7)
        });
        gpioa.moder.modify(|_, w| {
            w.moder2().alternate().moder3().alternate()
        });

        // 8 data bits, 1 stop bit
        usart2.cr2.write(|w| unsafe { w.stop().bits(0b00) });

        // Disable hardware flow control
        usart2.cr3.write(|w| w.rtse().bit(false).ctse().bit(false));

        // set baud rate
        let brr = u16(frequency::APB1 / baud_rate).unwrap();
        let fraction = u8(brr & 0b1111).unwrap();
        let mantissa = brr >> 4;
        usart2.brr.write(|w| unsafe {
            w.div_fraction().bits(fraction).div_mantissa().bits(
                mantissa,
            )
        });

        // enable peripheral, transmitter, receiver
        // enable RXNE event
        usart2.cr1.write(|w| {
            w.ue()
                .bit(true)
                .re()
                .bit(true)
                .te()
                .bit(true)
                .pce()
                .bit(false)
                .over8()
                .bit(false)
                .rxneie()
                .bit(true)
        });
    }

    /// Reads a byte from the RX buffer
    ///
    /// Returns `None` if the buffer is empty
    pub fn read(self) -> Result<u8> {
        let usart2 = self.0;

        if usart2.sr.read().rxne().bit() {
            // NOTE(read_volatile) the register is 9 bits big but we'll only
            // work with the first 8 bits
            Ok(unsafe {
                ptr::read_volatile(&usart2.dr as *const _ as *const u8)
            })
        } else {
            Err(Error { _0: () })
        }
    }

    /// Writes byte into the TX buffer
    ///
    /// Returns `Err` if the buffer is already full
    pub fn write(self, byte: u8) -> Result<()> {
        let usart2 = self.0;

        if usart2.sr.read().txe().bit() {
            unsafe { ptr::write_volatile(&usart2.dr as *const _ as *mut u8, byte) }
            Ok(())
        } else {
            Err(Error { _0: () })
        }
    }
}
