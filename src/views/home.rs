use crate::get_trains;
use dioxus::prelude::*;
use opentransportdata::{
    parse_formation_short_string, FormationResponse, Offer, StatusFlag, VehicleType,
};

const EW_IV_FIRST_CLASS_THRESHOLD: usize = 2;
const EW_IV_COUNT_THRESHOLD: usize = 3;
const CLOCK_ICON: Asset = asset!("/assets/clock.svg");
const LOCOMOTIVE_ICON: Asset = asset!("/assets/re460.svg");
const FAMILY_CAR_L_ICON: Asset = asset!("/assets/IC2000_FA_l.svg");
const FAMILY_CAR_R_ICON: Asset = asset!("/assets/IC2000_FA_r.svg");
const IC2000_ICON: Asset = asset!("/assets/IC2000.svg");
const EW_IV_ICON: Asset = asset!("/assets/ew_iv.svg");
const EW_IV_STEUERWAGEN_L_ICON: Asset = asset!("/assets/ew_iv_steuerwagen_l.svg");
const EW_IV_STEUERWAGEN_R_ICON: Asset = asset!("/assets/ew_iv_steuerwagen_r.svg");
const CLOSED_CAR_ICON: Asset = asset!("/assets/closed_car.svg");
const FIRST_CLASS_SVG: Asset = asset!("/assets/first_class.svg");
const SECOND_CLASS_SVG: Asset = asset!("/assets/second_class.svg");
// const DEKLASSIERT_CAR_ICON: Asset = asset!("/assets/deklassiert_car.svg");

const DEKLASSIERT_EW_IV_ICON: Asset = asset!("/assets/deklassiert_ew_iv.svg");

const RESTAURANT_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-ws.svg");
const WHEELCHAIR_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-rs.svg");
const BIKE_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-vo.svg");
const FAMILY_ZONE_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-fz.svg");
const BUSINESS_ZONE_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-bz.svg");
const RESERVED_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-r.svg");
const GROUP_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-reisegruppe.svg");
const LOW_FLOOR_SVG: Asset = asset!("/assets/sbb-icons-main/icons/sa-nf.svg");

// train number 600-649 is IC6/IC61
// train number 800-849 is IC8/IC81
// train number 950-999 is IC6/IC61
const IC8_SVG: Asset = asset!("/assets/sbb-icons-main/icons/ic-8.svg");
const IC81_SVG: Asset = asset!("/assets/sbb-icons-main/icons/ic-81.svg");
const IC6_SVG: Asset = asset!("/assets/sbb-icons-main/icons/ic-6.svg");
const IC61_SVG: Asset = asset!("/assets/sbb-icons-main/icons/ic-61.svg");
const IC_SVG: Asset = asset!("/assets/sbb-icons-main/icons/ic.svg");

#[component]
fn TrainView(train: FormationResponse) -> Element {
    let mut selected = use_signal(|| 0usize);

    let tabs = train
        .formations_at_scheduled_stops
        .iter()
        .map(|s| s.scheduled_stop.stop_point.name.clone())
        .collect::<Vec<_>>();

    let train_logo = if (800..850).contains(&train.train_meta_information.train_number) {
        if tabs.contains(&"Interlaken Ost".to_string()) {
            IC81_SVG
        } else {
            IC8_SVG
        }
    } else if (950..1000).contains(&train.train_meta_information.train_number)
        || (600..650).contains(&train.train_meta_information.train_number)
    {
        if tabs.contains(&"Interlaken Ost".to_string()) {
            IC61_SVG
        } else {
            IC6_SVG
        }
    } else {
        IC_SVG
    };

    rsx! {
        div { class: "tabs",

            div { class: "logo-row",
                img { src: train_logo, class: "app-logo" }
                "Nr {train.train_meta_information.train_number}"
            }
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
                    let mut cars = parse_formation_short_string(&train.formations_at_scheduled_stops[selected()].formation_short.formation_short_string,
                    EW_IV_FIRST_CLASS_THRESHOLD, EW_IV_COUNT_THRESHOLD);

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

                    // filter out fictional and parked cars
                    cars = cars.iter().filter(|c| c.vehicle_type != VehicleType::Fictional && c.vehicle_type != VehicleType::Parked).cloned().collect::<Vec<_>>();

                    let train_length = cars.len();

                    let rendered_cars: Vec<(Asset, Vec<Asset>, bool, Option<u32>)> =
                        cars.iter().enumerate().filter_map(|(i,car)| {

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

                            let (mut icon, class_label, overlay_class) = match car.vehicle_type {
                                VehicleType::Fictional | VehicleType::Parked => return None,

                                VehicleType::Locomotive => (LOCOMOTIVE_ICON, None, "class-overlay"),

                                VehicleType::FirstClass  =>
                                    if car.offers.contains(&Offer::LowFloor) {
                                        (IC2000_ICON, Some("1"), "class-overlay")
                                    } else {
                                        (EW_IV_ICON, Some("1"), "class-overlay")
                                    },
                                VehicleType::DiningFirstClass =>
                                    {
                                        overlay_icons.push(RESTAURANT_SVG);
                                        if car.offers.contains(&Offer::LowFloor) {
                                            (IC2000_ICON, Some("1"), "class-overlay")
                                        } else {
                                            (EW_IV_ICON, Some("1"), "class-overlay")
                                        }
                                    }

                                VehicleType::SecondClass =>
                                    if car.offers.contains(&Offer::LowFloor) {
                                        (IC2000_ICON, Some("2"), "class-overlay")
                                    } else {
                                        if i == 0 {
                                            // this is the first car, so show the steuerwagen!
                                            (EW_IV_STEUERWAGEN_L_ICON, Some("2"), "class-overlay family-left")
                                        } else if i == train_length - 1 {
                                            // this is the last car, so show the steuerwagen!
                                            (EW_IV_STEUERWAGEN_R_ICON, Some("2"), "class-overlay family-right")
                                        } else {
                                            (EW_IV_ICON, Some("2"), "class-overlay")
                                        }
                                    },

                                VehicleType::DiningSecondClass =>
                                    {
                                        overlay_icons.push(RESTAURANT_SVG);
                                        if car.offers.contains(&Offer::LowFloor) {
                                            (IC2000_ICON, Some("2"), "class-overlay")
                                        } else {
                                            (EW_IV_ICON, Some("2"), "class-overlay")
                                        }
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
                                    {
                                        if car.offers.contains(&Offer::LowFloor) {
                                            (IC2000_ICON, Some("1/2"), "class-overlay")
                                        } else {
                                            (EW_IV_ICON, Some("1/2"), "class-overlay")
                                        }
                                    },
                                _ => (IC2000_ICON, None, "class-overlay"),
                            };

                            let is_family_right = icon == FAMILY_CAR_R_ICON  || icon == EW_IV_STEUERWAGEN_R_ICON;

                            // closed overrides icon + label
                            if car.status.contains(&StatusFlag::Closed) {
                                icon = CLOSED_CAR_ICON;
                                overlay_icons = vec![];
                            };

                            if car.status.contains(&StatusFlag::Deklassiert) {
                                icon = DEKLASSIERT_EW_IV_ICON;
                            };

                            if car.status.contains(&StatusFlag::Reserved) {
                                overlay_icons.push(RESERVED_SVG);
                            };

                            if car.status.contains(&StatusFlag::GroupBoarding) {
                                overlay_icons.push(GROUP_SVG);
                            };

                            if car.offers.contains(&Offer::FamilyZone) {
                                if !overlay_icons.contains(&FAMILY_ZONE_SVG) {
                                    overlay_icons.push(FAMILY_ZONE_SVG);
                                }
                            };

                            if car.offers.contains(&Offer::LowFloor) {
                                overlay_icons.push(LOW_FLOOR_SVG);
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

#[component]
pub fn Home() -> Element {
    let trains_future = use_server_future(|| get_trains())?;

    let trains = match &*trains_future.read() {
        Some(Ok(trains)) => trains.clone(),
        Some(Err(_)) => return rsx! { div { "Failed to load trains" } },
        None => return rsx! { div { "Loading trains..." } },
    };

    let legend_items: Vec<(Asset, &str, &str, bool)> = vec![
        (
            LOCOMOTIVE_ICON,
            "Lokomotive",
            "legend-icon legend-icon--car",
            true,
        ),
        (
            FAMILY_CAR_L_ICON,
            "Familienwagen",
            "legend-icon legend-icon--car",
            true,
        ),
        (IC2000_ICON, "Wagen", "legend-icon legend-icon--car", true),
        (
            DEKLASSIERT_EW_IV_ICON,
            "Deklassiert",
            "legend-icon legend-icon--car",
            true,
        ),
        (
            CLOSED_CAR_ICON,
            "Geschlossener Wagen",
            "legend-icon legend-icon--car",
            true,
        ),
        (FIRST_CLASS_SVG, "1. Klasse", "legend-icon", false),
        (SECOND_CLASS_SVG, "2. Klasse", "legend-icon", false),
        (LOW_FLOOR_SVG, "Niederflur", "legend-icon", false),
        (RESTAURANT_SVG, "Restaurant", "legend-icon", false),
        (WHEELCHAIR_SVG, "Rollstuhl", "legend-icon", false),
        (BIKE_SVG, "Velo", "legend-icon", false),
        (FAMILY_ZONE_SVG, "Familienzone", "legend-icon", false),
        (BUSINESS_ZONE_SVG, "Business Zone", "legend-icon", false),
        (RESERVED_SVG, "Reserviert", "legend-icon", false),
        (GROUP_SVG, "Gruppenreservation", "legend-icon", false),
    ];

    rsx! {
        div { class: "app-header",
            div { class: "app-header__title", "deklassiert" }
            div { class: "app-header__badge", "2. Klasse" }
        }

        main { id: "trains",
            if trains.is_empty() {
                div { class: "container text-center mt-10", "Momentan sind keine Züge verfügbar..." }
            }

            for train in trains {
                TrainView { train: train }
            }
        }

        section { class: "legend",
            h2 { "Legende" }
            div { class: "legend-grid",
                for (icon, label, class_name, is_stacked) in legend_items {
                    div { class: if is_stacked { "legend-item legend-item--stacked" } else { "legend-item" },
                        if is_stacked {
                            span { class: "legend-label legend-label--top", "{label}" }
                            img { src: icon, class: "{class_name}" }
                        } else {
                            img { src: icon, class: "{class_name}" }
                            span { class: "legend-label", "{label}" }
                        }
                    }
                }
            }
            hr { class: "legend-separator" }
            div { class: "legend-text",
                h2 { class: "text-left", "deklassiert?" }
                p { class: "block text-left whitespace-pre-line",
                    "Zu Stosszeiten werden vermehrt Wagen (Einheitswagen IV) zur Unterstützung an bestehende IC2000-Kompositionen gekoppelt. Besonders an Freitagen und Wochenenden werden einzelne EW IV der 1. Klasse als Wagen der 2. Klasse geführt, also deklassiert."
                }
                p { class: "block text-left whitespace-pre-line",
                    "Mit den Daten von "
                    a {
                        href: "https://opentransportdata.swiss/de/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "external-link",
                        strong { "opentransportdata" }
                    }
                    " versuchen wir diese Wagen zu erkennen und entsprechend zu markieren."
                }
                p { class: "block text-left whitespace-pre-line",
                    "Alle Angaben ohne Gewähr."
                }
                p { class: "block text-left whitespace-pre-line",
                    "Diese Webseite wurde in Rust geschrieben und der Quellcode ist auf "
                    a { href: "https://github.com/hacknus/deklassiert", strong { "GitHub" } }
                    " verfügbar."
                }
                p { class: "block text-left whitespace-pre-line",
                    "© 2026 Linus Leo Stöckli"
                }
            }
        }
    }
}
