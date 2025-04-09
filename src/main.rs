#![no_main]
#![no_std]

// use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use microbit::{
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    display::blocking::Display,
    hal::Timer,
    hal::gpiote::Gpiote,
};

mod serial;

use serial::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    let gpiote = Gpiote::new(board.GPIOTE);
    
    let channel0 = gpiote.channel0();
    let channel1 = gpiote.channel1();

    channel0
        .input_pin(&board.buttons.button_a.degrade())
        .hi_to_lo();
    channel1
        .input_pin(&board.buttons.button_b.degrade())
        .hi_to_lo();
    
    channel0.reset_events();
    channel1.reset_events();

    // let button_a = board.buttons.button_a.degrade();
    // let button_b = board.buttons.button_b;

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut led_display;

    loop {
        // wfi();
        let a_pressed = channel0.is_event_triggered();
        let b_pressed = channel1.is_event_triggered();
        if a_pressed {
            channel0.reset_events();
            led_display = [[1u8; 5]; 5];
            serial.write(b'i').unwrap();
            serial.flush().unwrap();
            display.show(&mut timer, led_display, 100);
            led_display = [[0u8; 5]; 5];
            display.show(&mut timer, led_display, 100);
        }
        if b_pressed {
            channel1.reset_events();
            led_display = [[1u8; 5]; 5];
            serial.write(b'j').unwrap();
            serial.flush().unwrap();
            display.show(&mut timer, led_display, 100);
            led_display = [[0u8; 5]; 5];
            display.show(&mut timer, led_display, 100);
        }
    }
}
