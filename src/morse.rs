use crate::hal::{
    prelude::*,
    gpio::{
        Output, PushPull, gpioa::PA1 as Pin,
    }
};

use stm32f4xx_hal as hal;
use hal::dwt::Delay;

use panic_halt as _;


const DOT_MS: u32 = 200;
const DASH_MS: u32 = DOT_MS * 3;
const PAUSE_MS: u32 = 500;

pub struct MorseTranslator<'a> {
    pub output: &'a mut Pin<Output<PushPull>>,
    pub delay: &'a mut Delay,
}

pub enum Signal {
    Dot,
    Dash,
}

impl Signal {
    pub const fn get_ms(&self) -> u32 {
        match self {
            Self::Dot => DOT_MS,
            Self::Dash => DASH_MS,
        }
    }
}

impl<'a> MorseTranslator<'a> {
    pub fn play_string(&mut self, text: &'static str) {
        text
            .chars()
            .flat_map(|c| Self::get_signals_from_char(c))
            .for_each(|signal| {
                self.play_signal(&signal);
                self.delay.delay_ms(PAUSE_MS);
            });
    }

    pub fn play_signal(&mut self, signal: &Signal) {
        let delay = signal.get_ms();

        self.output.set_high().unwrap();
        self.delay.delay_ms(delay);
        self.output.set_low().unwrap();
    }

    fn get_signals_from_char(c: char) -> &'static [Signal] {
        use Signal::{Dot, Dash};

        match c.to_ascii_uppercase() {
            'A' => &[Dot, Dash],
            'B' => &[Dash, Dot, Dot],
            'C' => &[Dash, Dot, Dash, Dot],
            'D' => &[Dash, Dot, Dot],
            'E' => &[Dot],
            'F' => &[Dot, Dot, Dash, Dot],
            'G' => &[Dash, Dash, Dot],
            'H' => &[Dot, Dot, Dot, Dot],
            'I' => &[Dot, Dot],
            'J' => &[Dot, Dash, Dash, Dash],
            'K' => &[Dash, Dot, Dash],
            'L' => &[Dot, Dash, Dot, Dot],
            'M' => &[Dash, Dash],
            'N' => &[Dash, Dot],
            'O' => &[Dash, Dash, Dash],
            'P' => &[Dot, Dash, Dash, Dot],
            'Q' => &[Dash, Dash, Dot, Dash],
            'R' => &[Dot, Dash, Dot],
            'S' => &[Dot, Dot, Dot],
            'T' => &[Dash],
            'U' => &[Dot, Dot, Dash],
            'V' => &[Dot, Dot, Dot, Dash],
            'W' => &[Dot, Dash, Dash],
            'X' => &[Dash, Dot, Dot, Dash],
            'Y' => &[Dash, Dot, Dash, Dash],
            'Z' => &[Dash, Dash, Dot, Dot],

            _ => &[Dot, Dot, Dot, Dot, Dot],
        }
    }

}
