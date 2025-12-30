use dioxus::prelude::*;

pub struct KeyEvents<'a> {
    pub events: Vec<KeyEvent<'a>>
}

pub struct KeyEvent<'a> {
    pub on_key_down: Box<dyn FnMut(&Event<KeyboardData>) + 'a>
}

impl<'a> KeyEvents<'a> {
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn register<F>(&mut self, event: F)
    where F: FnMut(&Event<KeyboardData>) + 'a {
        self.events.push(KeyEvent { on_key_down: Box::new(event) })
    }

    pub fn act(&mut self, key_evt: &Event<KeyboardData>) {
        for evt in &mut self.events {
            (evt.on_key_down)(key_evt);
        }
    }
}
