use failure::*;

#[derive(Display, Debug)]
pub enum EventType {
    NONE = 0,
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    Apptick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

bitflags! {

    pub struct EventCategory : i32 {
        const NONE = 0;
        const EVENT_CATEGORY_APPLICATION    = bit!(1);
        const EVENT_CATEGORY_INPUT          = bit!(2);
        const EVENT_CATEGORY_KEYBOARD       = bit!(3);
        const EVENT_CATEGORY_MOUSE          = bit!(4);
        const EVENT_CATEGORY_MOUSEBUTTON    = bit!(5);
    }
}

#[allow(unused_variables)]
pub trait EventTrait {

    // Needs event dispatcher;
    fn get_event_type(&self) -> Result<&EventType, Error> { Ok(&EventType::NONE)}
    fn get_name(&self) -> Result<String, Error> { Ok(String::new()) }
    fn get_category_flags(&self) -> Result<&EventCategory, Error>;
    fn to_string(&self) -> Result<String, Error>;
    fn is_in_category(&self, category : &EventCategory) -> Result<bool, Error> { Ok(false)}
}

pub struct Event {

    event_type : EventType,
    flags : EventCategory
}

impl Event {

    pub fn new(event_type : EventType, flags: EventCategory) -> Result<Event, Error> {
        Ok(Event {event_type, flags})
    }

    pub fn get_category_flags(&self) -> Result<&EventCategory, Error> {
        Ok(&self.flags)
    }

    pub fn get_name(&self) -> Result<String, Error> {

        Ok(self.event_type.to_string())
    }

    pub fn get_event_type(&self) -> Result<&EventType, Error> {
        let value = &self.event_type;
        Ok(value)
    }

    pub fn is_in_category(&self, category : &EventCategory) -> Result<bool, Error> {

        Ok((category.to_owned() & self.get_category_flags()?.to_owned()) != EventCategory::NONE)
    }
}




