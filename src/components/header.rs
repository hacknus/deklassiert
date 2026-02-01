// File: `src/components/header.rs`
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "app-header",
            div { class: "app-header__title", "deklassiert" }
            div { class: "app-header__badge", "2. Klasse" }
        }
    }
}