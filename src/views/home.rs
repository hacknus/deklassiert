use crate::get_trains;
use dioxus::prelude::*;
use opentransportdata::{
    parse_formation_for_stop, FormationResponse, Offer, StatusFlag, VehicleIdentifier, VehicleType,
};

const ARROW_ICON: Asset = asset!("/assets/chevron-left-medium.svg");
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
    let mut selected = use_signal(|| select_current_or_next_stop(&train));
    let mut hover_vehicle = use_signal(|| None::<usize>);
    let mut pinned_vehicle = use_signal(|| None::<usize>);

    use_effect(move || {
        let _ = selected();
        hover_vehicle.set(None);
        pinned_vehicle.set(None);
    });

    let tabs_all = train
        .formations_at_scheduled_stops
        .iter()
        .map(|s| s.scheduled_stop.stop_point.name.clone())
        .collect::<Vec<_>>();

    let visible_stop_indices = visible_stop_indices(&train);
    if visible_stop_indices.is_empty() {
        return rsx! {
            div { class: "tabs",
                div { class: "logo-row",
                    img { src: IC_SVG, class: "app-logo" }
                    "Nr {train.train_meta_information.train_number}"
                }
                div { class: "tab-panel", "Keine passenden Halte gefunden." }
            }
        };
    }

    let selected_index = selected();
    let deklassiert_target_id = format!(
        "deklassiert-{}-{}",
        train.train_meta_information.train_number, selected_index
    );
    let deklassiert_target_id_effect = deklassiert_target_id.clone();

    use_effect(move || {
        let _ = selected_index;
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::{ScrollBehavior, ScrollIntoViewOptions, ScrollLogicalPosition};
            let Some(document) = web_sys::window().and_then(|w| w.document()) else {
                return;
            };
            if let Some(target) = document.get_element_by_id(&deklassiert_target_id_effect) {
                let mut opts = ScrollIntoViewOptions::new();
                opts.behavior(ScrollBehavior::Smooth);
                opts.inline(ScrollLogicalPosition::Center);
                opts.block(ScrollLogicalPosition::Nearest);
                target.scroll_into_view_with_scroll_into_view_options(&opts);
            }
        }
    });

    let active_vehicle = hover_vehicle().or(pinned_vehicle());
    let train_logo = if (800..850).contains(&train.train_meta_information.train_number) {
        if tabs_all.contains(&"Interlaken Ost".to_string()) {
            IC81_SVG
        } else {
            IC8_SVG
        }
    } else if (950..1000).contains(&train.train_meta_information.train_number)
        || (600..650).contains(&train.train_meta_information.train_number)
    {
        if tabs_all.contains(&"Interlaken Ost".to_string()) {
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
                for (i, stop_index) in visible_stop_indices.clone().into_iter().enumerate() {
                    li {
                        key: "{i}",
                        class: if selected() == stop_index { "tab active" } else { "tab" },
                        onclick: move |_| selected.set(stop_index),
                        "{train.formations_at_scheduled_stops[stop_index].scheduled_stop.stop_point.name}"
                    }
                }
            }

            div { class: "tab-panel",
                {
                    let mut cars = parse_formation_for_stop(&train, selected_index);

                    let stop = &train.formations_at_scheduled_stops[selected_index];

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


                    // filter out fictional and parked cars
                    cars = cars.iter().filter(|c| c.vehicle_type != VehicleType::Fictional && c.vehicle_type != VehicleType::Parked).cloned().collect::<Vec<_>>();

                    let train_length = cars.len();
                    let first_deklassiert_index = cars
                        .iter()
                        .position(|car| car.status.contains(&StatusFlag::Deklassiert));

                    let rendered_cars: Vec<(Asset, Vec<Asset>, bool, Option<u32>, Option<char>, Option<VehicleIdentifier>)> =
                        cars.iter().enumerate().filter_map(|(i,car)| {

                            let type_name = car
                                .vehicle_identifier
                                .as_ref()
                                .and_then(|v| v.type_code_name.as_deref())
                                .unwrap_or("");
                            let is_closed = car.status.contains(&StatusFlag::Closed);
                            let is_deklassiert = car.status.contains(&StatusFlag::Deklassiert);
                            let is_loco_name = type_name.starts_with("Re");
                            let is_steuerwagen_name =
                                type_name.starts_with("Bt") || type_name.starts_with("At");
                            let has_class = matches!(
                                car.vehicle_type,
                                VehicleType::FirstClass
                                    | VehicleType::DiningFirstClass
                                    | VehicleType::SecondClass
                                    | VehicleType::DiningSecondClass
                                    | VehicleType::FamilyCar
                                    | VehicleType::FirstAndSecondClass
                            );

                            let has_lowfloor =
                                car.offers.contains(&Offer::LowFloor) || type_name.contains("2E");
                            let has_bike = car.offers.contains(&Offer::BikeHooks)
                                || type_name.starts_with("Apm61")
                                || type_name.starts_with("Bpm61")
                                || type_name.starts_with("B3(503")
                                || type_name.starts_with("Bt4")
                                || type_name.starts_with("Bt(2E")
                                || type_name.to_lowercase().contains("velo");
                            let has_wheelchair = car.offers.contains(&Offer::Wheelchair)
                                || type_name.starts_with("Bpm61")
                                || type_name.starts_with("AD")
                                || type_name.starts_with("AS");
                            let has_business = car.offers.contains(&Offer::BusinessZone)
                                || type_name.starts_with("AD")
                                || type_name.starts_with("AS");
                            let has_family =
                                car.offers.contains(&Offer::FamilyZone) || type_name.contains("Fam");
                            let is_wra6_503 = type_name.starts_with("WRA6(503");
                            let has_restaurant =
                                !is_loco_name && (type_name.contains('W') || type_name.contains('R') && !type_name.to_lowercase().contains("era"));

                            // collect overlay icons
                            let mut overlay_icons = Vec::new();

                            let mut class_svg = match car.vehicle_type {
                                VehicleType::FirstClass | VehicleType::DiningFirstClass => Some(FIRST_CLASS_SVG),
                                VehicleType::SecondClass | VehicleType::DiningSecondClass | VehicleType::FamilyCar=> Some(SECOND_CLASS_SVG),
                                VehicleType::FirstAndSecondClass => None,
                                _ => None,
                            };
                            let mut class_label = match car.vehicle_type {
                                VehicleType::FirstClass | VehicleType::DiningFirstClass => Some("1"),
                                VehicleType::SecondClass | VehicleType::DiningSecondClass | VehicleType::FamilyCar => Some("2"),
                                VehicleType::FirstAndSecondClass => Some("1/2"),
                                _ => None,
                            };

                            if !is_closed && !has_class {
                                if type_name.starts_with("A") {
                                    class_svg = Some(if is_deklassiert { SECOND_CLASS_SVG } else { FIRST_CLASS_SVG });
                                    class_label = Some(if is_deklassiert { "2" } else { "1" });
                                } else if type_name.starts_with("B") {
                                    class_svg = Some(SECOND_CLASS_SVG);
                                    class_label = Some("2");
                                }
                            }

                            if !is_closed && has_restaurant {
                                if is_wra6_503 {
                                    class_svg = Some(FIRST_CLASS_SVG);
                                    class_label = Some("1");
                                } else {
                                    class_svg = Some(SECOND_CLASS_SVG);
                                    class_label = Some("2");
                                }
                            }

                            if let Some(class_svg) = class_svg {
                                overlay_icons.push(class_svg);
                            }

                            if !is_closed && (has_wheelchair || is_wra6_503) {
                                overlay_icons.push(WHEELCHAIR_SVG);
                            } else if car.offers.contains(&Offer::Wheelchair) {
                                overlay_icons.push(WHEELCHAIR_SVG);
                            }

                            if !is_closed && has_bike {
                                overlay_icons.push(BIKE_SVG);
                            } else if car.offers.contains(&Offer::BikeHooks) {
                                overlay_icons.push(BIKE_SVG);
                            }

                            if !is_closed && has_business {
                                overlay_icons.push(BUSINESS_ZONE_SVG);
                            } else if car.offers.contains(&Offer::BusinessZone) {
                                overlay_icons.push(BUSINESS_ZONE_SVG);
                            }

                            let (mut icon, overlay_class) = match car.vehicle_type {
                                VehicleType::Fictional | VehicleType::Parked => return None,

                                VehicleType::Locomotive => (LOCOMOTIVE_ICON, "class-overlay"),

                                VehicleType::FirstClass  =>
                                    if has_lowfloor {
                                        (IC2000_ICON, "class-overlay")
                                    } else {
                                        (EW_IV_ICON, "class-overlay")
                                    },
                                VehicleType::DiningFirstClass =>
                                    {
                                        if !overlay_icons.contains(&RESTAURANT_SVG) {
                                            overlay_icons.push(RESTAURANT_SVG);
                                        }
                                        if has_lowfloor {
                                            (IC2000_ICON, "class-overlay")
                                        } else {
                                            (EW_IV_ICON, "class-overlay")
                                        }
                                    }

                                VehicleType::SecondClass =>
                                    if has_lowfloor {
                                        (IC2000_ICON, "class-overlay")
                                    } else {
                                        (EW_IV_ICON, "class-overlay")
                                    },

                                VehicleType::DiningSecondClass =>
                                    {
                                        if !overlay_icons.contains(&RESTAURANT_SVG) {
                                            overlay_icons.push(RESTAURANT_SVG);
                                        }
                                        if has_lowfloor {
                                            (IC2000_ICON, "class-overlay")
                                        } else {
                                            (EW_IV_ICON, "class-overlay")
                                        }
                                    }

                                VehicleType::FamilyCar => {
                                    overlay_icons.push(FAMILY_ZONE_SVG);
                                    let left_blocked =
                                        i == 0 || car.access_to_previous_vehicle == Some(false);
                                    let right_blocked = i == train_length - 1
                                        || cars
                                            .get(i + 1)
                                            .map(|c| c.access_to_previous_vehicle == Some(false))
                                            .unwrap_or(true);
                                    let icon = if left_blocked && !right_blocked {
                                        FAMILY_CAR_L_ICON
                                    } else if right_blocked && !left_blocked {
                                        FAMILY_CAR_R_ICON
                                    } else if i == train_length - 1 {
                                        FAMILY_CAR_R_ICON
                                    } else {
                                        FAMILY_CAR_L_ICON
                                    };
                                    let overlay_class = if icon == FAMILY_CAR_R_ICON {
                                        "class-overlay family-right"
                                    } else {
                                        "class-overlay family-left"
                                    };
                                    (icon, overlay_class)
                                }

                                VehicleType::FirstAndSecondClass =>
                                    {
                                        if has_lowfloor {
                                            (IC2000_ICON, "class-overlay")
                                        } else {
                                            (EW_IV_ICON, "class-overlay")
                                        }
                                    },
                                _ => (IC2000_ICON, "class-overlay"),
                            };

                            if !is_closed && is_loco_name {
                                icon = LOCOMOTIVE_ICON;
                            } else if !is_closed && is_steuerwagen_name {
                                if type_name.starts_with("Bt") && type_name.contains("(2E") {
                                    let left_is_2e = if i > 0 {
                                        cars.get(i - 1)
                                            .and_then(|c| {
                                                c.vehicle_identifier
                                                    .as_ref()
                                                    .and_then(|v| v.type_code_name.as_deref())
                                            })
                                            .map(|t| t.contains("(2E"))
                                            .unwrap_or(false)
                                    } else {
                                        false
                                    };
                                    let right_is_2e = cars
                                        .get(i + 1)
                                        .and_then(|c| {
                                            c.vehicle_identifier
                                                .as_ref()
                                                .and_then(|v| v.type_code_name.as_deref())
                                        })
                                        .map(|t| t.contains("(2E"))
                                        .unwrap_or(false);
                                    if right_is_2e && !left_is_2e {
                                        icon = EW_IV_STEUERWAGEN_L_ICON;
                                    } else if left_is_2e && !right_is_2e {
                                        icon = EW_IV_STEUERWAGEN_R_ICON;
                                    } else {
                                        let left_blocked =
                                            i == 0 || car.access_to_previous_vehicle == Some(false);
                                        let right_blocked = i == train_length - 1
                                            || cars
                                                .get(i + 1)
                                                .map(|c| c.access_to_previous_vehicle == Some(false))
                                                .unwrap_or(true);
                                        if left_blocked && !right_blocked {
                                            icon = EW_IV_STEUERWAGEN_L_ICON;
                                        } else if right_blocked && !left_blocked {
                                            icon = EW_IV_STEUERWAGEN_R_ICON;
                                        } else if i == 0 {
                                            icon = EW_IV_STEUERWAGEN_L_ICON;
                                        } else if i == train_length - 1 {
                                            icon = EW_IV_STEUERWAGEN_R_ICON;
                                        }
                                    }
                                } else {
                                    let left_blocked =
                                        i == 0 || car.access_to_previous_vehicle == Some(false);
                                    let right_blocked = i == train_length - 1
                                        || cars
                                            .get(i + 1)
                                            .map(|c| c.access_to_previous_vehicle == Some(false))
                                            .unwrap_or(true);
                                    if left_blocked && !right_blocked {
                                        icon = EW_IV_STEUERWAGEN_L_ICON;
                                    } else if right_blocked && !left_blocked {
                                        icon = EW_IV_STEUERWAGEN_R_ICON;
                                    } else if i == 0 {
                                        icon = EW_IV_STEUERWAGEN_L_ICON;
                                    } else if i == train_length - 1 {
                                        icon = EW_IV_STEUERWAGEN_R_ICON;
                                    }
                                }
                            }

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

                            if !is_closed && has_family {
                                if !overlay_icons.contains(&FAMILY_ZONE_SVG) {
                                    overlay_icons.push(FAMILY_ZONE_SVG);
                                }
                            } else if car.offers.contains(&Offer::FamilyZone) {
                                if !overlay_icons.contains(&FAMILY_ZONE_SVG) {
                                    overlay_icons.push(FAMILY_ZONE_SVG);
                                }
                            };

                            if !is_closed && has_lowfloor {
                                overlay_icons.push(LOW_FLOOR_SVG);
                            } else if car.offers.contains(&Offer::LowFloor) {
                                overlay_icons.push(LOW_FLOOR_SVG);
                            };

                            if !is_closed && has_restaurant {
                                if !overlay_icons.contains(&RESTAURANT_SVG) {
                                    overlay_icons.push(RESTAURANT_SVG);
                                }
                            }

                            let identifier = car.vehicle_identifier.clone();
                            let _class_label = class_label;
                            let _overlay_class = overlay_class;
                            Some((icon, overlay_icons, is_family_right, car.order_number, car.sector, identifier))
                        })
                        .collect();

                    let mut sector_groups: Vec<(Option<char>, usize)> = Vec::new();
                    for (_, _, _, _, sector, _) in rendered_cars.iter() {
                        if let Some(last) = sector_groups.last_mut() {
                            if last.0 == *sector {
                                last.1 += 1;
                                continue;
                            }
                        }
                        sector_groups.push((*sector, 1));
                    }
                    let vehicle_count = rendered_cars.len();

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
                        div { class: "formation-row",
                            div { class: "sector-arrow-fixed",
                                img { src: ARROW_ICON, class: "clock-icon" }
                            }
                            div { class: "formation-scroll",
                                div { class: "sector-row", style: "grid-template-columns: repeat({vehicle_count}, var(--vehicle-width)); column-gap: var(--vehicle-gap);" ,
                                    for (i, (sector, count)) in sector_groups.iter().enumerate() {
                                        div {
                                            class: if i == 0 { "sector-block sector-block--first" } else { "sector-block" },
                                            style: "grid-column: span {count};",
                                            if let Some(letter) = sector {
                                                span { "{letter}" }
                                            }
                                        }
                                    }
                                }
                                div { class: "train-row", style: "grid-template-columns: repeat({vehicle_count}, var(--vehicle-width)); column-gap: var(--vehicle-gap);" ,
                                    for (index, (icon, overlay_icons, is_family_right, order_number, _, identifier)) in rendered_cars.iter().enumerate() {
                                        {
                                            let vehicle_id = if first_deklassiert_index == Some(index) {
                                                Some(deklassiert_target_id.clone())
                                            } else {
                                                None
                                            };
                                            rsx!(
                                                div {
                                                    class: "vehicle",
                                                    id: vehicle_id,

                                                    div { class: "car-number",
                                                        if let Some(num) = order_number {
                                                            "Wagen {num}"
                                                        }
                                                    }

                                                     div {
                                                        class: "vehicle-icon-wrapper",
                                                        onmouseenter: move |_| hover_vehicle.set(Some(index)),
                                                        onmouseleave: move |_| hover_vehicle.set(None),
                                                        onclick: move |_| {
                                                            if pinned_vehicle() == Some(index) {
                                                                pinned_vehicle.set(None);
                                                            } else {
                                                                pinned_vehicle.set(Some(index));
                                                            }
                                                            hover_vehicle.set(None);
                                                        },
                                                        img { src: *icon, class: "vehicle-icon" }

                                                    if active_vehicle == Some(index) {
                                                        if let Some(text) = format_vehicle_identifier(identifier) {
                                                            div { class: "vehicle-tooltip", "{text}" }
                                                        }
                                                    }

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
                                            )
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

fn format_vehicle_identifier(identifier: &Option<VehicleIdentifier>) -> Option<String> {
    let id = identifier.as_ref()?;
    let mut parts: Vec<String> = Vec::new();

    if let Some(name) = id.type_code_name.as_ref() {
        if !name.is_empty() {
            parts.push(name.clone());
        }
    }
    if let Some(evn) = id.evn.as_ref() {
        if !evn.is_empty() {
            parts.push(format!("EVN {evn}"));
        }
    }
    if let Some(parent) = id.parent_evn.as_ref() {
        if !parent.is_empty() {
            parts.push(format!("Parent {parent}"));
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" · "))
    }
}

fn visible_stop_indices(train: &FormationResponse) -> Vec<usize> {
    train
        .formations_at_scheduled_stops
        .iter()
        .enumerate()
        .filter(|(_, stop)| !stop.scheduled_stop.stop_type.contains('D'))
        .map(|(i, _)| i)
        .collect()
}

fn select_current_or_next_stop(train: &FormationResponse) -> usize {
    let visible = visible_stop_indices(train);
    if visible.is_empty() {
        return 0;
    }

    let now = chrono::Utc::now();
    for i in visible.iter().copied() {
        let stop = &train.formations_at_scheduled_stops[i];
        let arrival = stop
            .scheduled_stop
            .stop_time
            .arrival_time
            .map(|t| t.with_timezone(&chrono::Utc));
        let departure = stop
            .scheduled_stop
            .stop_time
            .departure_time
            .map(|t| t.with_timezone(&chrono::Utc));

        if let (Some(arrival), Some(departure)) = (arrival, departure) {
            if now >= arrival && now <= departure {
                return i;
            }
        }

        let next_time = departure.or(arrival);
        if let Some(next_time) = next_time {
            if next_time >= now {
                return i;
            }
        }
    }
    *visible.last().unwrap_or(&0)
}

fn sort_trains_by_selected_departure(trains: &mut Vec<FormationResponse>) {
    trains.sort_by_key(|train| {
        let idx = select_current_or_next_stop(train);
        let stop = &train.formations_at_scheduled_stops[idx];
        stop.scheduled_stop
            .stop_time
            .departure_time
            .or(stop.scheduled_stop.stop_time.arrival_time)
            .map(|t| t.with_timezone(&chrono::Utc))
    });
}

#[component]
pub fn Home() -> Element {
    let trains_future = use_server_future(|| get_trains())?;

    let mut trains = match &*trains_future.read() {
        Some(Ok(trains)) => trains.clone(),
        Some(Err(_)) => return rsx! { div { "Failed to load trains" } },
        None => return rsx! { div { "Loading trains..." } },
    };

    // filter trains to only those with deklassiert coaches
    trains = trains
        .iter()
        .filter(|train| {
            train
                .formations_at_scheduled_stops
                .iter()
                .enumerate()
                .any(|(i, _stop)| {
                    let vehicles = parse_formation_for_stop(train, i);
                    vehicles
                        .iter()
                        .filter(|v| v.status.contains(&StatusFlag::Deklassiert))
                        .count()
                        > 0
                })
        })
        .cloned()
        .collect();
    sort_trains_by_selected_departure(&mut trains);

    let legend_items: Vec<(Asset, &str, &str, bool)> = vec![
        (
            LOCOMOTIVE_ICON,
            "Lokomotive",
            "legend-icon legend-icon--car",
            true,
        ),
        (
            FAMILY_CAR_L_ICON,
            "Steuerwagen",
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
            a { class: "app-header__title", href: "/", "deklassiert" }
            div { class: "app-header__badge", "2. Klasse" }
        }

        main { id: "trains",
            if trains.is_empty() {
                div { class: "container text-center mt-10",
                    "Momentan sind leider keine deklassierten Wagen verfügbar, du kannst aber alle aktuellen IC6/61 und IC8/81 "
                    a { href: "/all", strong { "hier " } }
                    "anschauen."
                }
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
                    "Zu Stosszeiten werden vermehrt zusätzliche Wagen (Einheitswagen IV) zur Unterstützung an bestehende IC2020-Kompositionen gekoppelt. Besonders an Feiertagen und Wochenenden werden einzelne EW IV der 1. Klasse als Wagen der 2. Klasse geführt, also deklassiert."
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
                    " versuchen wir diese Wagen auf den IC6/61 und IC8/81 Linien zu erkennen und entsprechend zu markieren."
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
