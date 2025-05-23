#![no_std]
#![no_main]

use panic_halt as _;
use pcd8544_hal::Pcd8544;

static RUST_LOGO: &[u8; 504] = include_bytes!("logo.bin");

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut pcd8544 = pcd8544_hal::Pcd8544Gpio::new(
        /* sck/clk */ pins.d7.into_output(),
        /* miso/din */ pins.d6.into_output(),
        /* mosi/dc */ pins.d5.into_output(),
        /* cs/ce */ pins.d4.into_output(),
        /* rst */ Some(&mut pins.d3.into_output()),
        &mut arduino_hal::Delay::new(),
    );
    pcd8544.draw_buffer(RUST_LOGO);

    #[allow(clippy::empty_loop)]
    loop {}
}
