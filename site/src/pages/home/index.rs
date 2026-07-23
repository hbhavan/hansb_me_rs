use crate::components::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Section { text: "Full-Stack Software Development" }

        pre {
            code { class: "language-rs",
                {
                    r#"
                                let mut v = vec![0, 1, 2];
                                println!("{}", v);
                                "#
                }
            }
        }
    }
}
