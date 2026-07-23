use std::collections::VecDeque;

use dioxus::prelude::*;
use writ::data::*;
use crate::{components::*};
use crate::pages::projects::content::set_game::SetGame;

use super::{Render};

impl Render for SetGame {
    fn render(&self) -> Element {
        let mut set = use_signal(move || Set::new(Difficulty::Normal));

        use_effect(move || {
            set.write().initialize()
        });

        rsx! {
            div { class: "set-container",
                div {
                    Heading { text: "Set", size: HeadingSize::Medium }
                }
                div { class: "set-score", "Score: {set.read().score.to_string()}" }
                div { class: "set-header",
                    div {
                        match set.read().game_state() {
                            GameState::Ongoing(msg) => rsx! {
                                span { {msg} }
                            },
                            GameState::Concluded => rsx! {
                                span { "YOU WIN" }
                            },
                            GameState::Forfeit => rsx! {
                                span { "YOU LOSE" }
                            },
                        }
                    }
                    div {
                        button { onclick: move |_| on_submit(&mut set.write()), "Submit" }
                        button { onclick: move |_| on_clear(&mut set.write()), "Clear" }
                        button { onclick: move |_| test_board(&mut set.write()), "Check" }
                    }

                    div { class: "set-difficulty-buttons",
                        for difficulty in Difficulty::get_difficulties() {
                            button {
                                class: difficulty_selected(&set.read(), difficulty.clone()),
                                onclick: move |_| on_difficulty_click(&mut set.write(), difficulty.clone()),
                                "{difficulty.to_string()}"
                            }
                        }
                    }
                }

                div { class: "set-board",
                    for (i, card) in set.read().board.iter().enumerate() {
                        div {
                            class: vec!["set-card", card_selected(&set.read(), i), is_blank(card)].join(" "),
                            key: "{i.to_string()}",
                            onclick: move |_| select_card(&mut set.write(), i),
                            {card.render()}
                        }
                    }
                }
            }
        }
    }
}

impl Render for Card {
    fn render(&self) -> Element {
        let color = self.color.get_color();
        let desc = self.desc();

        rsx! {
            div { class: color,
                for i in 0..3 {
                    span { id: "{i} - {desc}", {self.get_row(i)} }
                }
            }
        }
    }
}

fn on_submit(set: &mut Set) {
    set.submit()
}

fn on_clear(set: &mut Set) {
    set.selected = VecDeque::with_capacity(3)
}

fn select_card(set: &mut Set, index: usize) {
    let card = set.board.get(index).unwrap_or(&Card::blank()).clone();
    let desc = card.desc();

    info!("{index:<4} - {desc}");

    set.select_card(index);
}

fn card_selected<'a>(set: &Set, index: usize) -> &'a str {
    match set.selected.contains(&index) {
        true => "selected",
        false => ""
    }
}

fn difficulty_selected<'a>(set: &Set, difficulty: Difficulty) -> &'a str {
    match set.difficulty == difficulty {
        true => "selected",
        false => ""
    }
}

fn is_blank<'a>(card: &Card) -> &'a str {
    match card.is_blank() {
        true => "blank",
        false => ""
    }
}

fn on_difficulty_click(set: &mut Set, difficulty: Difficulty) {
    *set = Set::new(difficulty);

    set.initialize()
}

fn test_board(set: &mut Set) {
    let mut distinct: Vec<Card> = vec![];
    let mut duplicates: Vec<Card> = vec![];

    for card in set.board.iter() {
        if distinct.contains(card) {
            duplicates.push(card.clone());
        } else {
            distinct.push(card.clone());
        }
    }

    info!("Distinct: {x}", x = distinct.len());
    info!("Duplicates: {y}", y = duplicates.len());
}
