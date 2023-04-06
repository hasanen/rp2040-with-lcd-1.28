//  WIP

#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_graphics::prelude::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    pixelcolor::Rgb565,
    prelude::*,
    text::{Baseline, Text},
};
use fugit::RateExtU32;

use rp2040_hal::{gpio::Pins, i2c::I2C, pac, Sio};
// use embedded_hal::adc::OneShot;
// use rp2040_hal::{clocks::init_clocks_and_plls, pac, sio::Sio, gpio::Pin};
// use ssd1306::{prelude::*, Builder};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    // Set up clocks and SIO
    let mut peripherals = pac::Peripherals::take().unwrap();
    let sio = Sio::new(peripherals.SIO);
    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    // Set up ADC on GPIO26
    // let mut adc = pac.ADC;
    // // let adcccc_pin =Pin::
    // let adc_pin = rp2040_hal::gpio::Pin::new(26).into_adc(&mut adc);
    // let mut adc = OneShot::new(adc_pin);

    let mut i2c = I2C::i2c1(
        peripherals.I2C1,
        pins.gpio6.into_mode(), // sda
        pins.gpio7.into_mode(), // scl
        400.kHz(),
        &mut peripherals.RESETS,
        125_000_000.Hz(),
    );
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(
        "Hello world!",
        Point::new(50, 50),
        text_style,
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(70, 70), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

// fn read_temperature(adc: &mut OneShot<rp2040_hal::gpio::Adc<rp2040_hal::pac::ADC>>) -> f32 {
//     // Convert ADC reading to voltage
//     let reading = adc.read().unwrap();
//     let voltage = reading as f32 * 3.3 / 65535.0;

//     // Convert voltage to temperature in degrees Celsius
//     let temperature = (voltage - 0.706) / 0.001721;
//     temperature
// }
