use dioxus::{logger::tracing::info, prelude::*};

use crate::data::{handle_key_down, KeyboardNav};

#[derive(Clone, Debug, PartialEq)]
pub struct GridCell {
    x: usize,
    y: usize,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyGridProp {
    width: usize,
    height: usize,
    current: Option<(i32, i32)>,
    grid: Vec<GridCell>,
}

impl GridCell {
    pub fn new(coords: (usize, usize), content: &str) -> Self {
        Self {
            x: coords.0,
            y: coords.1,
            content: content.to_string(),
        }
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl KeyGridProp {
    pub fn new(width: usize, height: usize) -> Self {
        let center = ((width / 2) as i32, (height / 2) as i32);

        let grid = (0..width * height)
            .into_iter()
            .map(|i| {
                let (x, y) = (i % width, i / height);
                let content = &(i + 1).to_string();
                GridCell::new((x, y), content)
            })
            .collect();

        Self {
            width,
            height,
            grid,
            current: Some(center),
        }
    }

    pub fn is_active(&self, cell: &GridCell) -> bool {
        self.current
            .map(|(x, y)| (x as usize, y as usize))
            .is_some_and(|curr| cell.coords() == curr)
    }

    pub fn grid(&self) -> &Vec<GridCell> {
        &self.grid
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    pub fn move_current(&mut self, coords: (i32, i32)) {
        self.current
            .map(|(x, y)| (x + coords.0, y + coords.1))
            .filter(|(x, y)| self.in_bounds(*x, *y))
            .map(move |curr| self.current = Some(curr));
    }

    pub fn handle_key(&mut self, e: Event<KeyboardData>) {
        handle_key_down(e, self);
    }
}

impl KeyboardNav for KeyGridProp {
    fn on_j_press(&mut self) {
        self.move_current((0, 1));
    }
    fn on_k_press(&mut self) {
        self.move_current((0, -1));
    }
    fn on_l_press(&mut self) {
        self.move_current((1, 0));
    }
    fn on_h_press(&mut self) {
        self.move_current((-1, 0));
    }
    fn on_space_press(&mut self) {}
}

#[component]
pub fn KeyGrid() -> Element {
    let mut g = use_signal(|| KeyGridProp::new(3, 3));

    rsx! {
        div {
            tabindex: 0,
            class: "key-listener",
            onkeydown: move |e| g.write().handle_key(e),
            div {
                class: "key-grid",
                for cell in g.read().grid() {
                    div {
                        class: "key-grid-cell",
                        {cell.content.clone()},
                        div {
                            if g.read().is_active(cell) {
                                {"!!!"}
                            }
                        }
                    }
                }
            }
        }
    }
}
