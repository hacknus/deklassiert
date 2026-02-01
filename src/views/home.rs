use dioxus::prelude::*;
use crate::get_tabs;

#[component]
pub fn Home() -> Element {
    let tabs_future = use_server_future(|| get_tabs())?;

    let mut selected = use_signal(|| 0usize);

    let tabs = match &*tabs_future.read() {
        Some(Ok(tabs)) => tabs.clone(),
        Some(Err(_e)) => {
            return rsx! { div { "Failed to load tabs" } };
        }
        None => {
            return rsx! { div { "Loading stops..." } };
        }
    };

    rsx! {
        div { class: "app-header",
            div { class: "app-header__title", "deklassiert" }
            div { class: "app-header__badge", "2. Klasse" }
        }

        main { id: "hero",
            div { class: "tabs",
                ul { class: "tab-list",
                    for (i, t) in tabs.iter().enumerate() {
                        li {
                            key: "{i}",
                            class: if selected() == i { "tab active" } else { "tab" },
                            onclick: move |_| selected.set(i),
                            "{t}"
                        }
                    }
                }

                div { class: "tab-panel",
                    "Content for: {tabs[selected()].clone()}"
                }
            }
        }
    }
}