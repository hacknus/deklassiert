use crate::get_train;
use dioxus::prelude::*;

const CLOCK_ICON: Asset = asset!("/assets/clock.svg");

#[component]
pub fn Home() -> Element {
    let train_future = use_server_future(|| get_train())?;

    let mut selected = use_signal(|| 0usize);

    let train = match &*train_future.read() {
        Some(Ok(trains)) => trains.clone(),
        Some(Err(_e)) => {
            return rsx! { div { "Failed to load train" } };
        }
        None => {
            return rsx! { div { "Loading train..." } };
        }
    };

    let tabs = train
        .formations_at_scheduled_stops
        .iter()
        .map(|s| s.scheduled_stop.stop_point.name.clone())
        .collect::<Vec<_>>();

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
                    {
                        let stop = &train.formations_at_scheduled_stops[selected()];

                        let arrival = stop
                            .scheduled_stop
                            .stop_time
                            .arrival_time
                            .map(|t| t.format("%H:%M").to_string());

                        let departure = stop
                            .scheduled_stop
                            .stop_time
                            .departure_time
                            .map(|t| t.format("%H:%M").to_string());

                        rsx! {
                            div { class: "time-row",

                                span { class: "time-item",
                                    span { "Gleis {stop.scheduled_stop.track}" }
                                }

                                if let Some(a) = arrival {
                                    span { class: "time-item",
                                        img { src: CLOCK_ICON, class: "clock-icon" }
                                        span { "Ankunft {a}" }
                                    }
                                }

                                if let Some(d) = departure {
                                    span { class: "time-item",
                                        img { src: CLOCK_ICON, class: "clock-icon" }
                                        span { "Abfahrt {d}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
