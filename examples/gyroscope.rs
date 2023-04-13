#![no_std]
#![no_main]

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio, pac, pwm,
    sio::Sio,
    spi,
    watchdog::Watchdog,
};
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Baseline,
    text::Text,
};
use fugit::RateExtU32;
use rp_pico as bsp;

use rp2040_with_lcd_128::qmi8658c;

#[rp2040_hal::entry]
fn main() -> ! {
    let screen_width = 240;
    let screen_height = 240;
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let sio = Sio::new(peripherals.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let rst_pin = pins.gpio12.into_push_pull_output();
    let dc_pin = pins.gpio8.into_push_pull_output();

    let _spi_sclk = pins.gpio10.into_mode::<gpio::FunctionSpi>();
    let _spi_mosi = pins.gpio11.into_mode::<gpio::FunctionSpi>();
    let spi_cs = pins.gpio9.into_push_pull_output();

    // Create an SPI driver instance for the SPI1 device
    let spi = spi::Spi::<_, _, 8>::new(peripherals.SPI1);
    // Exchange the uninitialised SPI driver for an initialised one
    let spi = spi.init(
        &mut peripherals.RESETS,
        clocks.peripheral_clock.freq(),
        8u32.MHz(),
        &embedded_hal::spi::MODE_0,
    );
    let spi_interface = SPIInterface::new(spi, dc_pin, spi_cs);

    // initialize PWM for backlight
    let pwm_slices = pwm::Slices::new(peripherals.PWM, &mut peripherals.RESETS);

    // Configure PWM6
    let mut pwm = pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    let mut channel = pwm.channel_b;
    channel.output_to(pins.led);

    // Create display driver
    let mut display = gc9a01a::GC9A01A::new(spi_interface, rst_pin, channel);
    // Bring out of reset
    display.reset(&mut delay).unwrap();
    // Turn on backlight
    display.set_backlight(55000);
    // Initialize registers
    display.initialize(&mut delay).unwrap();
    // Clear the screen
    display.clear(Rgb565::BLACK).unwrap();

    let circle_style = PrimitiveStyleBuilder::new()
        .stroke_width(2)
        .stroke_color(Rgb565::CSS_AQUAMARINE)
        .build();

    // screen outline for the round 1.28 inch Waveshare display
    Circle::new(Point::new(1, 1), 238)
        .into_styled(circle_style)
        .draw(&mut display)
        .unwrap();

    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);

    loop {
        let mut text = Text::new(
            "Hello Rust!Hello Rust!Hello Rust!",
            Point::new(0, 0),
            text_style,
        );

        let text_size = text.bounding_box().size;
        text.position = Point::new(
            (screen_width / 2) - (text_size.width as i32 / 2),
            (screen_height / 2) + (text_size.height as i32 / 2),
        );

        text.draw(&mut display).unwrap();
        delay.delay_ms(500);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
