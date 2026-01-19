use dioxus::prelude::*;

#[component]
pub fn ErrorPage() -> Element {
    rsx! {
        main {
            id: "error",
            h1 { "An error has occured." }
            p { "Please contact Hans about this" }
        }
    }
}
