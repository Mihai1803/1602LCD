#![no_std]
#![no_main]

//use core::panic::PanicInfo;
use embassy_executor::Spawner;

// USB driver
use embassy_rp::usb::{Driver, InterruptHandler as UsbInterruptHandler};
use embassy_rp::{bind_interrupts, peripherals::USB};
use log::info;

// time
use embassy_time::Delay;

//LCD
use embassy_rp::i2c::{I2c, Config};
use ag_lcd::{Cursor, LcdDisplay};
use port_expander::dev::pcf8574::Pcf8574;
use panic_halt as _;


bind_interrupts!(struct Irqs {
    // Use for the serial over USB driver
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
});

// The task used by the serial port driver over USB
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

const DISPLAY_FREQ: u32 = 200_000;


#[embassy_executor::main]
async fn main(spawner: Spawner) {

    let peripherals = embassy_rp::init(Default::default());

    // Start the serial port over USB driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Initiate SDA and SCL pins
    let sda = peripherals.PIN_4;
    let scl = peripherals.PIN_5;
    
    // Initiate Delay
    let delay = Delay;
    let mut config = Config::default();
    config.frequency = DISPLAY_FREQ;

    // Initiate I2C
    let i2c = I2c::new_blocking(peripherals.I2C0, scl, sda, config.clone());
    let mut i2c_expander = Pcf8574::new(i2c, true, true, true);

    // Initiate LCD
    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay)
    .with_cursor(Cursor::Off)
    .with_reliable_init(10000)
    .build();

    // Write to LCD
    lcd.print("Hello World");


    loop {
      info!("Working");
      embassy_time::Timer::after_secs(10).await;
    }


}

