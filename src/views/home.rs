use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let tabs = vec![
        "Brig".to_string(),
        "Spiez".to_string(),
        "Thun".to_string(),
        "Bern".to_string(),
    ];

    let mut selected = use_signal(|| 0usize);

    rsx! {

        link { rel: "stylesheet", href: asset!("/assets/styling/main.css") }

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