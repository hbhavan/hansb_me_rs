use dioxus::{html::KeyboardData, logger::tracing::info, prelude::Event};

pub trait KeyboardNav {
    fn on_j_press(&mut self) {}
    fn on_k_press(&mut self) {}
    fn on_l_press(&mut self) {}
    fn on_h_press(&mut self) {}
    fn on_space_press(&mut self) {}
}

pub fn handle_key_down<K: KeyboardNav>(evt: Event<KeyboardData>, key_nav: &mut K) {
    use dioxus::events::Key::Character;
    let key = evt.key();

    if let Character(k) = key {
        match k.to_uppercase().as_str() {
            "J" => key_nav.on_j_press(),
            "K" => key_nav.on_k_press(),
            "H" => key_nav.on_h_press(),
            "L" => key_nav.on_l_press(),
            "SPACE" => key_nav.on_space_press(),
            _ => (),
        };
    }
}

#[allow(dead_code)]
pub struct KeyLogger {
    page: String,
    location: String,
}

#[allow(dead_code)]
impl KeyLogger {
    pub fn new(page: &str, location: &str) -> Self {
        Self {
            page: page.to_string(),
            location: location.to_string(),
        }
    }

    pub fn log(&self, key: &'static str) {
        info!(
            "{page}\t{location}\t - {key} pressed",
            page = self.page,
            location = self.location
        )
    }
}

impl KeyboardNav for KeyLogger {
    fn on_j_press(&mut self) {
        self.log("J");
    }
    fn on_k_press(&mut self) {
        self.log("K");
    }
    fn on_l_press(&mut self) {
        self.log("L");
    }
    fn on_h_press(&mut self) {
        self.log("H");
    }
    fn on_space_press(&mut self) {
        self.log("SPACE");
    }
}
