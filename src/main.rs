#![deny(unsafe_code)]
#![no_main]
#![no_std]


mod morse;
use morse::MorseTranslator;

use crate::hal::{
    dwt::DwtExt,
    prelude::*,
    stm32,
};
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;



#[entry]
fn main() -> ! {
    // Retrieve peripherials
    let peripherals = stm32::Peripherals::take().unwrap();
    let core_peripherals = cortex_m::peripheral::Peripherals::take().unwrap();

    // Split GPIO block A into independant pins and registers
    let gpio_a = peripherals.GPIOA.split();

    // Split GPIO block C into independant pins and registers
    let gpio_c = peripherals.GPIOC.split();

    // Configure C14 pin to work as pulled up input 
    //
    // GND <-- 10kΩ <--- |BTN| ---> C14
    //
    let btn = gpio_c.pc14.into_pull_up_input();

    // Configure A1 pin to work as output
    //
    // GND <-- 10kΩ <--- |LED| ---> A1
    //
    let mut led =  gpio_a.pa1.into_push_pull_output();

    // The RCC peripheral is used to control the internal peripherals, as well as the reset signals and clock distribution. 
    let rcc = peripherals.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let dwt = core_peripherals.DWT.constrain(core_peripherals.DCB, clocks);
    let mut delay = dwt.delay();

    let mut morse_translator = MorseTranslator{
        delay: &mut delay,
        output: &mut led,
    };

    loop {
        // Low means that button is down
        if btn.is_low().unwrap() {
            morse_translator.play_string("hello");
        }
    }
}
