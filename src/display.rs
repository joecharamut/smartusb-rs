use std::error::Error;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use sh1106::{prelude::*, Builder, displayrotation::DisplayRotation};
use rppal::{gpio::Gpio, gpio::OutputPin, i2c::I2c};

pub type OledDisplay = GraphicsMode<I2cInterface<I2c>>;

pub struct Display {
    disp: OledDisplay,
    rst: OutputPin,
}

impl Display {
    pub fn new(flip: bool) -> Result<Display, Box<dyn Error>> {
        let i2c = I2c::new()?;
        let gpio = Gpio::new()?;

        Ok(Display {
            disp: Builder::new()
                .with_rotation(match flip {
                    true => DisplayRotation::Rotate180,
                    false => DisplayRotation::Rotate0,
                })
                .connect_i2c(i2c)
                .into(),
            rst: gpio.get(25)?.into_output(),
        })
    }

    pub fn init(&mut self) {
        self.rst.set_high();
        self.disp.init().unwrap();
        self.flush();
    }

    pub fn clear(&mut self) {
        self.disp.clear();
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32) {
        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();
        Text::with_baseline(text, Point::new(x, y), style, Baseline::Top)
            .draw(&mut self.disp)
            .unwrap();
    }

    pub fn flush(&mut self) {
        self.disp.flush().unwrap();
    }
}


