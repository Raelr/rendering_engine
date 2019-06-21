use crate::events::*;
use crate::events::event::Event;
use EventType;
use failure::*;

#[derive()]
pub struct MouseMovedEvent {
    event: Event,
    m_mouse_x : f64,
    m_mouse_y : f64
}

impl MouseMovedEvent {

    fn new(m_mouse_x : f64, m_mouse_y : f64) -> Result<MouseMovedEvent, Error> {

        let flags = event::EventCategory::EVENT_CATEGORY_MOUSE | event::EventCategory::EVENT_CATEGORY_INPUT;

        let event = MouseMovedEvent {
            event : event!(EventType::MouseMoved, flags),
            m_mouse_x,
            m_mouse_y
        };

        Ok(event)
    }

    fn get_x(&self) -> &f64 {
        &self.m_mouse_x
    }

    fn get_y(&self) -> &f64 {
        &self.m_mouse_y
    }
}

impl event::EventTrait for MouseMovedEvent {

    fn get_event_type(&self) -> &EventType {
        self.event.get_event_type()
    }

    fn get_name(&self) -> String {
        self.event.get_name()
    }

    fn get_category_flags(&self) -> &event::EventCategory {
        self.event.get_category_flags()
    }

    fn to_string(&self) -> String {

        let debug = format!("{}: {}, {}", self.get_name(), self.get_x(), self.get_y());

        debug
    }

    fn is_in_category(&self, category: &event::EventCategory) -> bool {
        self.event.is_in_category(category)
    }
}
