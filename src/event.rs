
#[derive(Debug, PartialEq)]
pub enum EventType {
    InputEvent,
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub input_data: Option<InputEventData>
}

impl Event {
    pub fn new_input_event(key: InputKey) -> Event {
        Event {
            event_type: EventType::InputEvent,
            input_data: Some(InputEventData {
                key: Some(key),
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InputKey {
    JoyUp,
    JoyDown,
    JoyLeft,
    JoyRight,
    JoyPress,
    Key1,
    Key2,
    Key3,
}

#[derive(Debug)]
pub struct InputEventData {
    pub key: Option<InputKey>,
}

