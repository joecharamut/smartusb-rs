use std::{error::Error, sync::mpsc};

use rppal::{gpio::Gpio, gpio::InputPin, gpio::Trigger, gpio::Level};

use crate::event::{Event, EventType, InputKey};
use closure::closure;

pub struct Input {
    pin_joyup: InputPin,
    pin_joydown: InputPin,
    pin_joyleft: InputPin,
    pin_joyright: InputPin,
    pin_joypress: InputPin,

    pin_key1: InputPin,
    pin_key2: InputPin,
    pin_key3: InputPin,
}

impl Input {
    pub fn new() -> Result<Input, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        let up = gpio.get(6)?.into_input_pullup();
        let down = gpio.get(19)?.into_input_pullup();
        let left = gpio.get(5)?.into_input_pullup();
        let right = gpio.get(26)?.into_input_pullup();
        let press = gpio.get(13)?.into_input_pullup();

        let key1 = gpio.get(21)?.into_input_pullup();
        let key2 = gpio.get(20)?.into_input_pullup();
        let key3 = gpio.get(16)?.into_input_pullup();

        Ok(Input {
            pin_joyup: up,
            pin_joydown: down,
            pin_joyleft: left,
            pin_joyright: right,
            pin_joypress: press,
            pin_key1: key1,
            pin_key2: key2,
            pin_key3: key3,
        })
    }

    pub fn connect_interrupts(&mut self, event_handler: mpsc::Sender<Event>) -> Result<(), rppal::gpio::Error> {
        self.pin_joyup.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::JoyUp)).unwrap();
        }))?;

        self.pin_joydown.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::JoyDown)).unwrap();
        }))?;

        self.pin_joyleft.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::JoyLeft)).unwrap();
        }))?;

        self.pin_joyright.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::JoyRight)).unwrap();
        }))?;

        self.pin_joypress.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::JoyPress)).unwrap();
        }))?;

        self.pin_key1.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::Key1)).unwrap();
        }))?;

        self.pin_key2.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::Key2)).unwrap();
        }))?;

        self.pin_key3.set_async_interrupt(Trigger::RisingEdge, closure!(clone event_handler, |_level: Level| {
            event_handler.send(Event::new_input_event(InputKey::Key3)).unwrap();
        }))?;

        Ok(())
    }
}



