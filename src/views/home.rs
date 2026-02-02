use crate::get_train;
use dioxus::prelude::*;
use opentransportdata::{parse_formation_short_string, Offer, StatusFlag, VehicleType};

const CLOCK_ICON: Asset = asset!("/assets/clock.svg");
const LOCOMOTIVE_ICON: Asset = asset!("/assets/re460.svg");
const FAMILY_CAR_L_ICON: Asset = asset!("/assets/IC2000_FA_l.svg");
const FAMILY_CAR_R_ICON: Asset = asset!("/assets/IC2000_FA_r.svg");
const CAR_ICON: Asset = asset!("/assets/car.svg");
const CLOSED_CAR_ICON: Asset = asset!("/assets/closed_car.svg");
const FIRST_CLASS_SVG: Asset = asset!("/assets/first_class.svg");
const SECOND_CLASS_SVG: Asset = asset!("/assets/second_class.svg");

const DEKLASSIERT_CAR: Asset = asset!("/assets/deklassiert_car.svg");
const RESTAURANT_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-ws.svg");
const WHEELCHAIR_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-rs.svg");
const BIKE_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-vo.svg");
const FAMILY_ZONE_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-fz.svg");
const BUSINESS_ZONE_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-bz.svg");

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
                        let cars = parse_formation_short_string(&train.formations_at_scheduled_stops[selected()].formation_short.formation_short_string);

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


                        let mut prev_had_lowfloor = false;

                        let rendered_cars: Vec<(Asset, Vec<Asset>, bool, Option<u32>)> =
                            cars.iter().filter_map(|car| {
                                dbg!(car);

                                // collect overlay icons
                                let mut overlay_icons = Vec::new();

                                let class_svg = match car.vehicle_type {
                                    VehicleType::FirstClass | VehicleType::DiningFirstClass => Some(FIRST_CLASS_SVG),
                                    VehicleType::SecondClass | VehicleType::DiningSecondClass | VehicleType::FamilyCar=> Some(SECOND_CLASS_SVG),
                                    _ => None,
                                };

                                if let Some(class_svg) = class_svg {
                                    overlay_icons.push(class_svg);
                                }

                                if car.offers.contains(&Offer::Wheelchair) {
                                    overlay_icons.push(WHEELCHAIR_SVG);
                                }

                                if car.offers.contains(&Offer::BikeHooks) {
                                    overlay_icons.push(BIKE_SVG);
                                }

                                if car.offers.contains(&Offer::BusinessZone) {
                                    overlay_icons.push(BUSINESS_ZONE_SVG);
                                }

                                let (icon, class_label, overlay_class) = match car.vehicle_type {
                                    VehicleType::Fictional | VehicleType::Parked => return None,

                                    VehicleType::Locomotive => (LOCOMOTIVE_ICON, None, "class-overlay"),

                                    VehicleType::FirstClass  =>
                                        (CAR_ICON, Some("1"), "class-overlay"),

                                    VehicleType::DiningFirstClass =>
                                        {
                                            overlay_icons.push(RESTAURANT_SVG);
                                            (CAR_ICON, Some("1"), "class-overlay")
                                        }

                                    VehicleType::SecondClass =>
                                        (CAR_ICON, Some("2"), "class-overlay"),

                                    VehicleType::DiningSecondClass =>
                                        {
                                            overlay_icons.push(RESTAURANT_SVG);
                                            (CAR_ICON, Some("2"), "class-overlay")
                                        }

                                    VehicleType::FamilyCar => {
                                        overlay_icons.push(FAMILY_ZONE_SVG);
                                        if prev_had_lowfloor {
                                            (FAMILY_CAR_R_ICON, Some("2"), "class-overlay family-right")
                                        } else {
                                            (FAMILY_CAR_L_ICON, Some("2"), "class-overlay family-left")
                                        }
                                    }

                                    VehicleType::FirstAndSecondClass =>
                                        (CAR_ICON, Some("1/2"), "class-overlay"),

                                    _ => (CAR_ICON, None, "class-overlay"),
                                };

                                let is_family_right = matches!(car.vehicle_type, VehicleType::FamilyCar) && prev_had_lowfloor;

                                // closed overrides icon + label
                                let (icon, overlay_icons) = if car.status.contains(&StatusFlag::Closed) {
                                    (CLOSED_CAR_ICON, vec![])
                                } else {
                                    (icon, overlay_icons)
                                };

                                prev_had_lowfloor = car.offers.contains(&Offer::LowFloor);

                                Some((icon, overlay_icons, is_family_right, car.order_number))
                            })
                            .collect();

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
                            div { class: "train-row",
                                for (icon, overlay_icons, is_family_right, order_number) in rendered_cars.iter() {
                                    div { class: "vehicle",

                                        div { class: "car-number",
                                            if let Some(num) = order_number {
                                                "Wagen {num}"
                                            }
                                        }

                                         div { class: "vehicle-icon-wrapper",
                                            img { src: *icon, class: "vehicle-icon" }

                                            if !overlay_icons.is_empty() {
                                                div {
                                                    class: if *is_family_right {
                                                        "overlay-icons family-right"
                                                    } else {
                                                        "overlay-icons"
                                                    },

                                                    for icon in overlay_icons.iter() {
                                                        img {
                                                            src: *icon,
                                                            class: "overlay-icon"
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
                }
            }
        }
    }
}
