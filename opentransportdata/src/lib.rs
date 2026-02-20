use chrono::{DateTime, Datelike, FixedOffset, TimeZone};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

use quick_xml::events::Event;
// rust
use quick_xml::Reader;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use VehicleType::FamilyCar;

#[derive(Debug, Clone, PartialEq)]
pub enum StatusFlag {
    Closed,        // -
    GroupBoarding, // >
    Reserved,      // =
    OpenUnserved,  // %
    Deklassiert,   // not in formation string, needs to be populated separately
}

#[derive(Debug, Clone, PartialEq)]
pub enum VehicleType {
    FirstClass,          // "1"
    SecondClass,         // "2"
    FirstAndSecondClass, // "12"
    FamilyCar,           // "FA"
    SleepingCar,         // "WL"
    Restaurant,          // "WR"
    DiningFirstClass,    // "W1"
    DiningSecondClass,   // "W2"
    Locomotive,          // "LK"
    BaggageCar,          // "D"
    Fictional,           // "F"
    Classless,           // "K"
    Parked,              // "X"
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Offer {
    Wheelchair,   // BHP
    BusinessZone, // BZ
    FamilyZone,   // FZ
    Stroller,     // KW
    LowFloor,     // NF
    BikeHooks,    // VH
    BikeReserved, // VR
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub sector: Option<char>,
    pub status: Vec<StatusFlag>,
    pub no_passage_left: bool,
    pub no_passage_right: bool,
    pub vehicle_type: VehicleType,
    pub order_number: Option<u32>,
    pub offers: Vec<Offer>,
    pub vehicle_identifier: Option<VehicleIdentifier>,
    pub access_to_previous_vehicle: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormationResponse {
    #[serde(default, deserialize_with = "null_to_empty")]
    pub vehicle_journey_type: String,
    #[serde(default, deserialize_with = "null_to_empty")]
    pub last_update: String,
    pub journey_meta_information: JourneyMetaInformation,
    pub train_meta_information: TrainMetaInformation,
    pub formations_at_scheduled_stops: Vec<FormationAtScheduledStop>,
    #[serde(default)]
    pub formations: Vec<Formation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JourneyMetaInformation {
    #[serde(default, deserialize_with = "null_to_empty")]
    pub operation_date: String,

    #[serde(rename = "SJYID")]
    #[serde(default, deserialize_with = "null_to_empty")]
    pub sjyid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrainMetaInformation {
    pub train_number: u32,
    #[serde(default, deserialize_with = "null_to_empty")]
    pub to_code: String,
    #[serde(default, deserialize_with = "null_to_empty")]
    pub runs: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormationAtScheduledStop {
    pub scheduled_stop: ScheduledStop,
    pub formation_short: FormationShort,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledStop {
    pub stop_point: StopPoint,
    pub stop_modifications: u32,
    #[serde(default, deserialize_with = "null_to_empty")]
    pub stop_type: String,
    pub stop_time: StopTime,
    #[serde(default, deserialize_with = "null_to_empty")]
    pub track: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StopPoint {
    pub uic: u32,
    #[serde(default, deserialize_with = "null_to_empty")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StopTime {
    pub arrival_time: Option<DateTime<FixedOffset>>,
    pub departure_time: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormationShort {
    #[serde(default, deserialize_with = "null_to_empty")]
    pub formation_short_string: String,
    pub vehicle_goals: Vec<VehicleGoal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VehicleGoal {
    pub from_vehicle_at_position: u32,
    pub to_vehicle_at_position: u32,
    pub destination_stop_point: StopPoint,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Formation {
    #[serde(default)]
    pub formation_vehicles: Vec<FormationVehicle>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormationVehicle {
    #[serde(default)]
    pub vehicle_identifier: Option<VehicleIdentifier>,
    pub position: u32,
    pub number: u32,
    #[serde(default)]
    pub formation_vehicle_at_scheduled_stops: Vec<FormationVehicleAtScheduledStop>,
    #[serde(default)]
    pub vehicle_properties: Option<VehicleProperties>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VehicleProperties {
    #[serde(default)]
    pub trolley_status: Option<TrolleyStatus>,
    #[serde(default)]
    pub number1class: u32,
    #[serde(default)]
    pub number2class: u32,
    #[serde(default)]
    pub number_bike_hooks: u32,
    #[serde(default)]
    pub bike_platform: bool,
    #[serde(default)]
    pub low_floor_trolley: bool,
    #[serde(default)]
    pub accessibility_properties: Option<AccessibilityProperties>,
    #[serde(default)]
    pub picto_properties: Option<PictoProperties>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccessibilityProperties {
    #[serde(default)]
    pub number_wheelchair_spaces: u32,
    #[serde(default)]
    pub wheelchair_toilet: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PictoProperties {
    #[serde(default)]
    pub bike_picto: bool,
    #[serde(default)]
    pub family_zone_picto: bool,
    #[serde(default)]
    pub business_zone_picto: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TrolleyStatus {
    #[serde(rename = "GeschlossenTechnisch")]
    GeschlossenTechnisch,
    #[serde(rename = "GeschlossenBetrieblich")]
    GeschlossenBetrieblich,
    #[serde(rename = "RestaurantUnbedient")]
    RestaurantUnbedient,
    #[serde(rename = "RestaurantUnbedientDeklassiert")]
    RestaurantUnbedientDeklassiert,
    #[serde(rename = "Deklassiert")]
    Deklassiert,
    #[serde(rename = "Normal")]
    Normal,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormationVehicleAtScheduledStop {
    pub stop_point: StopPoint,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub sectors: Option<String>,
    #[serde(default)]
    pub access_to_previous_vehicle: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VehicleIdentifier {
    #[serde(default)]
    pub type_code: Option<u32>,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub type_code_name: Option<String>,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub build_type_code: Option<String>,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub country_code: Option<String>,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub vehicle_number: Option<String>,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub check_number: Option<String>,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub evn: Option<String>,
    #[serde(default, deserialize_with = "null_to_empty_opt")]
    pub parent_evn: Option<String>,
    #[serde(default)]
    pub position: Option<u32>,
}

fn null_to_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.unwrap_or_default())
}

fn null_to_empty_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.filter(|s| !s.is_empty()))
}

pub fn parse_formation_json(json: &str) -> Result<FormationResponse, serde_json::Error> {
    serde_json::from_str::<FormationResponse>(json)
}

pub fn get_vehicle_information(
    train: &FormationResponse,
) -> HashMap<u32, (bool, FormationVehicle)> {
    let mut map = HashMap::new();
    for formation in train.formations.iter() {
        for vehicle in formation.formation_vehicles.iter() {
            match vehicle
                .vehicle_properties
                .as_ref()
                .and_then(|props| props.trolley_status.clone())
            {
                Some(TrolleyStatus::Deklassiert)
                | Some(TrolleyStatus::RestaurantUnbedientDeklassiert) => {
                    map.insert(vehicle.number, (true, vehicle.clone()));
                }
                _ => {
                    map.insert(vehicle.number, (false, vehicle.clone()));
                }
            }
        }
    }

    map
}

fn deklassiert_by_number(
    train: &FormationResponse,
) -> HashMap<u32, (bool, Option<VehicleIdentifier>)> {
    let mut map = HashMap::new();
    if let Some(formation) = train.formations.first() {
        for vehicle in formation.formation_vehicles.iter() {
            let is_deklassiert = matches!(
                vehicle
                    .vehicle_properties
                    .as_ref()
                    .and_then(|props| props.trolley_status.clone()),
                Some(TrolleyStatus::Deklassiert)
                    | Some(TrolleyStatus::RestaurantUnbedientDeklassiert)
            );
            map.insert(
                vehicle.number,
                (is_deklassiert, vehicle.vehicle_identifier.clone()),
            );
        }
    }
    map
}

fn identifiers_by_position(train: &FormationResponse) -> Vec<Option<VehicleIdentifier>> {
    let mut map: BTreeMap<u32, Option<VehicleIdentifier>> = BTreeMap::new();
    for formation in train.formations.iter() {
        for vehicle in formation.formation_vehicles.iter() {
            map.entry(vehicle.position)
                .or_insert(vehicle.vehicle_identifier.clone());
        }
    }
    let max_pos = map.keys().copied().max().unwrap_or(0);
    let mut identifiers = vec![None; max_pos as usize];
    for (pos, identifier) in map {
        if pos == 0 {
            continue;
        }
        let index = pos.saturating_sub(1) as usize;
        if let Some(slot) = identifiers.get_mut(index) {
            *slot = identifier;
        }
    }
    identifiers
}

fn positions_by_number(train: &FormationResponse) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for formation in train.formations.iter() {
        for vehicle in formation.formation_vehicles.iter() {
            if vehicle.number > 0 {
                map.insert(vehicle.number, vehicle.position);
            }
        }
    }
    map
}

fn sectors_by_position_for_stop(train: &FormationResponse, stop_uic: u32) -> Vec<Option<char>> {
    let mut map: BTreeMap<u32, Option<char>> = BTreeMap::new();
    for formation in train.formations.iter() {
        for vehicle in formation.formation_vehicles.iter() {
            let sector = vehicle
                .formation_vehicle_at_scheduled_stops
                .iter()
                .find(|s| s.stop_point.uic == stop_uic)
                .and_then(|s| s.sectors.as_ref())
                .and_then(|s| {
                    let first = s.split(',').next()?.trim();
                    first.chars().next()
                });
            map.insert(vehicle.position, sector);
        }
    }
    let max_pos = map.keys().copied().max().unwrap_or(0);
    let mut sectors = vec![None; max_pos as usize];
    for (pos, sector) in map {
        if pos == 0 {
            continue;
        }
        let index = pos.saturating_sub(1) as usize;
        if let Some(slot) = sectors.get_mut(index) {
            *slot = sector;
        }
    }
    sectors
}

fn zero_number_identifiers_by_position(
    train: &FormationResponse,
) -> Vec<Option<VehicleIdentifier>> {
    let mut map: BTreeMap<u32, Option<VehicleIdentifier>> = BTreeMap::new();
    for formation in train.formations.iter() {
        for vehicle in formation.formation_vehicles.iter() {
            if vehicle.number == 0 {
                let identifier = vehicle.vehicle_identifier.clone();
                let type_code_name_lower = identifier
                    .as_ref()
                    .and_then(|v| v.type_code_name.as_deref())
                    .unwrap_or("")
                    .to_lowercase();
                let is_locomotive_identifier = identifier
                    .as_ref()
                    .and_then(|v| v.type_code)
                    .map(|code| code == 1057)
                    .unwrap_or(false)
                    || type_code_name_lower.starts_with("re");
                if is_locomotive_identifier {
                    map.entry(vehicle.position).or_insert(identifier);
                }
            }
        }
    }
    map.into_values().collect()
}

fn parse_vehicle_type(s: &str) -> VehicleType {
    match s {
        "1" => VehicleType::FirstClass,
        "2" => VehicleType::SecondClass,
        "12" => VehicleType::FirstAndSecondClass,
        "FA" => FamilyCar,
        "WL" => VehicleType::SleepingCar,
        "WR" => VehicleType::Restaurant,
        "W1" => VehicleType::DiningFirstClass,
        "W2" => VehicleType::DiningSecondClass,
        "LK" => VehicleType::Locomotive,
        "D" => VehicleType::BaggageCar,
        "F" => VehicleType::Fictional,
        "K" => VehicleType::Classless,
        "X" => VehicleType::Parked,
        other => VehicleType::Unknown(other.to_string()),
    }
}

fn parse_offer(s: &str) -> Offer {
    match s {
        "BHP" => Offer::Wheelchair,
        "BZ" => Offer::BusinessZone,
        "FZ" => Offer::FamilyZone,
        "KW" => Offer::Stroller,
        "NF" => Offer::LowFloor,
        "VH" => Offer::BikeHooks,
        "VR" => Offer::BikeReserved,
        other => Offer::Unknown(other.to_string()),
    }
}

fn vehicle_type_from_detailed(vehicle: &FormationVehicle) -> VehicleType {
    let type_name = vehicle
        .vehicle_identifier
        .as_ref()
        .and_then(|v| v.type_code_name.as_deref())
        .unwrap_or("");

    let (n1, n2) = vehicle
        .vehicle_properties
        .as_ref()
        .map(|p| (p.number1class, p.number2class))
        .unwrap_or((0, 0));

    if vehicle.number == 0 && n1 == 0 && n2 == 0 {
        return VehicleType::Locomotive;
    }

    if type_name.starts_with("WR") {
        return VehicleType::DiningSecondClass;
    }

    if type_name.contains("Fam") {
        return FamilyCar;
    }

    if n1 > 0 && n2 > 0 {
        return VehicleType::FirstAndSecondClass;
    }

    if n1 > 0 {
        return VehicleType::FirstClass;
    }

    if n2 > 0 {
        return VehicleType::SecondClass;
    }

    if type_name.is_empty() {
        VehicleType::Unknown("Unknown".to_string())
    } else {
        VehicleType::Unknown(type_name.to_string())
    }
}

fn offers_from_detailed(vehicle: &FormationVehicle) -> Vec<Offer> {
    let mut offers = Vec::new();
    let Some(props) = vehicle.vehicle_properties.as_ref() else {
        return offers;
    };

    if props.low_floor_trolley {
        offers.push(Offer::LowFloor);
    }

    let has_bike = props.number_bike_hooks > 0
        || props.bike_platform
        || props
            .picto_properties
            .as_ref()
            .map(|p| p.bike_picto)
            .unwrap_or(false);
    if has_bike {
        offers.push(Offer::BikeHooks);
    }

    let has_wheelchair = props
        .accessibility_properties
        .as_ref()
        .map(|a| a.number_wheelchair_spaces > 0 || a.wheelchair_toilet)
        .unwrap_or(false);
    if has_wheelchair {
        offers.push(Offer::Wheelchair);
    }

    if props
        .picto_properties
        .as_ref()
        .map(|p| p.business_zone_picto)
        .unwrap_or(false)
    {
        offers.push(Offer::BusinessZone);
    }

    if props
        .picto_properties
        .as_ref()
        .map(|p| p.family_zone_picto)
        .unwrap_or(false)
    {
        offers.push(Offer::FamilyZone);
    }

    offers
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ShortEl {
    Loco,
    Car(u32),
}

#[derive(Debug, Clone)]
struct ShortItem {
    kind: ShortEl,
    sector: Option<char>,
    is_family: bool,
}

fn short_string_items(formation_short: &str) -> Vec<ShortItem> {
    if formation_short.is_empty() {
        return Vec::new();
    }

    let num_re = Regex::new(r":(\d+)").ok();

    let mut items = Vec::new();
    let mut current_sector: Option<char> = None;

    for part in formation_short.split(',') {
        let tok = part.trim();
        if tok.is_empty() {
            continue;
        }

        if let Some(idx) = tok.find('@') {
            if let Some(letter) = tok[idx + 1..].chars().next() {
                if letter.is_ascii_uppercase() {
                    current_sector = Some(letter);
                }
            }
        }

        let is_family = tok.contains("FA");

        if tok.contains("LK") && !tok.contains(':') {
            items.push(ShortItem {
                kind: ShortEl::Loco,
                sector: current_sector,
                is_family: false,
            });
            continue;
        }

        let mut last_num: Option<u32> = None;
        if let Some(re) = &num_re {
            for cap in re.captures_iter(tok) {
                if let Some(num_str) = cap.get(1) {
                    if let Ok(num) = num_str.as_str().parse::<u32>() {
                        last_num = Some(num);
                    }
                }
            }
        }
        if let Some(num) = last_num {
            items.push(ShortItem {
                kind: ShortEl::Car(num),
                sector: current_sector,
                is_family,
            });
        } else if tok.contains("LK") {
            items.push(ShortItem {
                kind: ShortEl::Loco,
                sector: current_sector,
                is_family: false,
            });
        }
    }

    items
}

fn short_string_numbers_seq(formation_short: &str) -> Vec<u32> {
    short_string_items(formation_short)
        .into_iter()
        .filter_map(|i| match i.kind {
            ShortEl::Car(n) => Some(n),
            _ => None,
        })
        .collect()
}

fn short_string_car1_left(formation_short: &str) -> Option<bool> {
    let seq = short_string_numbers_seq(formation_short);
    if seq.is_empty() {
        return None;
    }

    let idx = seq.iter().position(|n| *n == 1)?;
    let last_idx = seq.len().saturating_sub(1);
    Some(idx <= last_idx.saturating_sub(idx))
}

fn short_string_numbers(formation_short: &str) -> BTreeSet<u32> {
    short_string_numbers_seq(formation_short)
        .into_iter()
        .collect()
}

pub fn parse_formation_for_stop(train: &FormationResponse, stop_index: usize) -> Vec<Vehicle> {
    let stop = &train.formations_at_scheduled_stops[stop_index];
    let stop_uic = stop.scheduled_stop.stop_point.uic;
    let short_str = &stop.formation_short.formation_short_string;
    let short_items = short_string_items(short_str);
    let short_seq = short_string_numbers_seq(short_str);
    let short_numbers = short_string_numbers(short_str);

    let Some(formation) = train.formations.first() else {
        return Vec::new();
    };

    let mut formation_vehicles = formation.formation_vehicles.clone();
    formation_vehicles.sort_by_key(|v| v.position);

    let stop_index_by_uic: HashMap<u32, usize> = train
        .formations_at_scheduled_stops
        .iter()
        .enumerate()
        .map(|(idx, s)| (s.scheduled_stop.stop_point.uic, idx))
        .collect();
    let current_stop_index = stop_index_by_uic.get(&stop_uic).copied().unwrap_or(0);

    let mut out = Vec::new();
    for formation_vehicle in formation_vehicles {
        let earliest_idx = formation_vehicle
            .formation_vehicle_at_scheduled_stops
            .iter()
            .filter_map(|s| stop_index_by_uic.get(&s.stop_point.uic).copied())
            .min();
        if earliest_idx.map_or(false, |idx| current_stop_index < idx) {
            let type_name = formation_vehicle
                .vehicle_identifier
                .as_ref()
                .and_then(|v| v.type_code_name.as_deref())
                .unwrap_or("");
            let is_family_like = type_name.contains("Fam")
                || type_name.starts_with("Bt")
                || type_name.starts_with("At");
            let allow_from_short = formation_vehicle.number > 0
                && short_numbers.contains(&formation_vehicle.number)
                || (formation_vehicle.number == 0
                    && is_family_like
                    && short_str.contains("FA"));
            if !allow_from_short {
                continue;
            }
        }

        let mut stop_info = formation_vehicle
            .formation_vehicle_at_scheduled_stops
            .iter()
            .find(|s| s.stop_point.uic == stop_uic);

        if stop_info.is_none() {
            stop_info = formation_vehicle
                .formation_vehicle_at_scheduled_stops
                .iter()
                .filter_map(|s| {
                    let idx = stop_index_by_uic.get(&s.stop_point.uic).copied()?;
                    (idx <= current_stop_index).then_some((idx, s))
                })
                .max_by_key(|(idx, _)| *idx)
                .map(|(_, s)| s);
        }

        let sector = stop_info
            .and_then(|info| info.sectors.as_ref())
            .and_then(|s| s.split(',').next())
            .and_then(|s| s.trim().chars().next());

        let mut status = Vec::new();
        if let Some(props) = formation_vehicle.vehicle_properties.as_ref() {
            if matches!(
                props.trolley_status,
                Some(TrolleyStatus::Deklassiert)
                    | Some(TrolleyStatus::RestaurantUnbedientDeklassiert)
            ) {
                status.push(StatusFlag::Deklassiert);
            }
        }

        let is_loco_identifier = formation_vehicle
            .vehicle_identifier
            .as_ref()
            .and_then(|v| v.type_code_name.as_deref())
            .map(|name| name.starts_with("Re"))
            .unwrap_or(false)
            || formation_vehicle
                .vehicle_identifier
                .as_ref()
                .and_then(|v| v.type_code)
                .map(|code| code == 1057)
                .unwrap_or(false);

        let vehicle = Vehicle {
            sector,
            status,
            no_passage_left: false,
            no_passage_right: false,
            vehicle_type: vehicle_type_from_detailed(&formation_vehicle),
            order_number: if is_loco_identifier {
                None
            } else if formation_vehicle.number > 0 {
                Some(formation_vehicle.number)
            } else {
                None
            },
            offers: offers_from_detailed(&formation_vehicle),
            vehicle_identifier: formation_vehicle.vehicle_identifier.clone(),
            access_to_previous_vehicle: stop_info.and_then(|info| info.access_to_previous_vehicle),
        };

        out.push(vehicle);
    }

    let is_loco_id = |vehicle: &Vehicle| {
        vehicle
            .vehicle_identifier
            .as_ref()
            .and_then(|v| v.type_code_name.as_deref())
            .map(|name| name.starts_with("Re"))
            .unwrap_or(false)
            || vehicle
                .vehicle_identifier
                .as_ref()
                .and_then(|v| v.type_code)
                .map(|code| code == 1057)
                .unwrap_or(false)
    };

    if !short_items.is_empty() {
        let mut by_number: HashMap<u32, Vehicle> = HashMap::new();
        let mut locos: Vec<Vehicle> = Vec::new();
        let mut extras: Vec<Vehicle> = Vec::new();
        let mut family_pool: Vec<Vehicle> = Vec::new();

        for v in out.into_iter() {
            if v.vehicle_type == VehicleType::Locomotive || is_loco_id(&v) {
                locos.push(v);
                continue;
            }
            let is_family = v
                .vehicle_identifier
                .as_ref()
                .and_then(|id| id.type_code_name.as_deref())
                .map(|name| name.contains("Fam"))
                .unwrap_or(false);
            if is_family {
                family_pool.push(v);
                continue;
            }
            if let Some(num) = v.order_number {
                by_number.insert(num, v);
            } else {
                extras.push(v);
            }
        }

        let mut used_numbers: BTreeSet<u32> = BTreeSet::new();
        let mut used_loco = 0usize;
        let mut ordered: Vec<Vehicle> = Vec::new();

        for item in short_items.iter() {
            match item.kind {
                ShortEl::Loco => {
                    if let Some(mut loco) = locos.get(used_loco).cloned() {
                        loco.order_number = None;
                        loco.sector = item.sector.or(loco.sector);
                        ordered.push(loco);
                        used_loco += 1;
                    }
                }
                ShortEl::Car(num) => {
                    if used_numbers.contains(&num) {
                        continue;
                    }
                    used_numbers.insert(num);
                    if item.is_family && !family_pool.is_empty() {
                        let mut v = family_pool.remove(0);
                        v.order_number = Some(num);
                        v.sector = item.sector.or(v.sector);
                        ordered.push(v);
                    } else if let Some(mut v) = by_number.remove(&num) {
                        v.order_number = Some(num);
                        v.sector = item.sector.or(v.sector);
                        ordered.push(v);
                    } else if item.is_family && !extras.is_empty() {
                        let mut v = extras.remove(0);
                        v.order_number = Some(num);
                        v.sector = item.sector.or(v.sector);
                        ordered.push(v);
                    }
                }
            }
        }

        ordered.extend(family_pool.into_iter());
        for (_, v) in by_number.into_iter() {
            ordered.push(v);
        }
        ordered.extend(extras.into_iter());
        ordered.extend(locos.into_iter().skip(used_loco).map(|mut v| {
            v.order_number = None;
            v
        }));

        out = ordered;
    } else {
        for vehicle in out.iter_mut() {
            if vehicle.vehicle_type == VehicleType::Locomotive || is_loco_id(vehicle) {
                vehicle.order_number = None;
            }
        }
    }

    out
}

fn parse_vehicle(raw: &str, sector: Option<char>) -> Option<Vehicle> {
    if raw.is_empty() {
        return None;
    }

    let mut chars = raw.chars().peekable();

    let mut status = Vec::new();
    loop {
        match chars.peek() {
            Some('-') => {
                chars.next();
                status.push(StatusFlag::Closed);
            }
            Some('>') => {
                chars.next();
                status.push(StatusFlag::GroupBoarding);
            }
            Some('=') => {
                chars.next();
                status.push(StatusFlag::Reserved);
            }
            Some('%') => {
                chars.next();
                status.push(StatusFlag::OpenUnserved);
            }
            _ => break,
        }
    }

    let rest: String = chars.collect();
    let body = rest.trim();

    if body.is_empty() {
        return None;
    }

    let (vehicle_part_raw, offers_part) = body.split_once('#').unwrap_or((body, ""));

    let no_passage_left = vehicle_part_raw.contains('(') || offers_part.contains('(');
    let no_passage_right = vehicle_part_raw.contains(')') || offers_part.contains(')');

    let vehicle_part_clean = vehicle_part_raw
        .replace('(', "")
        .replace(')', "")
        .trim()
        .to_string();

    let offers_part_clean = offers_part
        .replace('(', "")
        .replace(')', "")
        .trim()
        .to_string();

    let (vehicle_type_str, order_number) =
        if let Some((typ, ord)) = vehicle_part_clean.split_once(':') {
            (typ.trim(), ord.parse::<u32>().ok())
        } else {
            (vehicle_part_clean.as_str(), None)
        };

    let vehicle_type = parse_vehicle_type(vehicle_type_str);

    let offers = if offers_part_clean.is_empty() {
        Vec::new()
    } else {
        offers_part_clean
            .split(';')
            .filter(|s| !s.is_empty())
            .map(parse_offer)
            .collect()
    };

    Some(Vehicle {
        sector,
        status,
        no_passage_left,
        no_passage_right,
        vehicle_type,
        order_number,
        offers,
        vehicle_identifier: None,
        access_to_previous_vehicle: None,
    })
}

#[cfg(feature = "native-client")]
pub fn get_train_formation(
    train_id: i32,
    year: i32,
    month: u32,
    day: u32,
    token: &str,
) -> Result<FormationResponse, String> {
    let base_url = "https://api.opentransportdata.swiss/formation/v1";

    let url = format!(
        "{}/formations_full?evu=SBBP&operationDate={}-{}-{}&trainNumber={}",
        base_url, year, month, day, train_id
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", token)
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "API request failed with status: {}",
            response.status()
        ));
    }

    let json_text = response
        .text()
        .map_err(|e| format!("Failed to read response text: {}", e))?;

    parse_formation_json(&json_text).map_err(|e| format!("JSON parsing error: {}", e))
}

/// Strip namespace declarations and prefixes.
fn strip_namespaces(xml: &str) -> String {
    let re_xmlns = Regex::new(r#"\sxmlns(:\w+)?="[^"]+""#).unwrap();
    let re_tag = Regex::new(r#"<(/?)([A-Za-z0-9_]+):"#).unwrap();
    let re_attr = Regex::new(r#"(\s)([A-Za-z0-9_]+):"#).unwrap();

    let s = re_xmlns.replace_all(xml, "");
    let s = re_tag.replace_all(&s, "<$1");
    let s = re_attr.replace_all(&s, "$1");
    s.to_string()
}

fn handle_text(
    text: &str,
    elem_stack: &Vec<String>,
    current_train_number: &mut Option<String>,
    current_departure_time: &mut Option<DateTime<chrono::Utc>>,
    current_latest_arrival: &mut Option<DateTime<chrono::Utc>>,
    in_service_departure: bool,
    in_service_arrival: bool,
) {
    if text.is_empty() {
        return;
    }

    if let Some(current) = elem_stack.last() {
        let lower = current.to_lowercase();

        if lower.ends_with("trainnumber") || lower.ends_with("operatingnumber") {
            *current_train_number = Some(text.to_string());
        }

        if in_service_departure && lower.ends_with("timetabledtime") {
            if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
                *current_departure_time = Some(dt.with_timezone(&chrono::Utc));
            }
        }

        if in_service_arrival && lower.ends_with("timetabledtime") {
            if let Ok(dt) = DateTime::parse_from_rfc3339(text) {
                let dt = dt.with_timezone(&chrono::Utc);
                let replace = current_latest_arrival.map_or(true, |cur| dt > cur);
                if replace {
                    *current_latest_arrival = Some(dt);
                }
            }
        }
    }
}

/// Event-driven parser that keeps an element stack and collects unique train numbers.
/// Works across quick-xml versions by avoiding methods that may not exist.
pub fn parse_train_numbers(xml: &str) -> Vec<String> {
    let cleaned = strip_namespaces(xml);
    let mut reader = Reader::from_str(&cleaned);

    let mut buf = Vec::new();
    let mut elem_stack: Vec<String> = Vec::new();
    let mut numbers: BTreeSet<String> = BTreeSet::new();

    let now_utc = chrono::Utc::now();
    let tz = chrono_tz::Europe::Zurich;
    let today_local = now_utc.with_timezone(&tz).date_naive();
    let tomorrow_local = today_local.succ_opt().unwrap();
    let end_of_today = tz
        .with_ymd_and_hms(
            tomorrow_local.year(),
            tomorrow_local.month(),
            tomorrow_local.day(),
            4,
            0,
            0,
        )
        .single()
        .unwrap()
        .with_timezone(&chrono::Utc);

    // per StopEvent state
    let mut current_train_number: Option<String> = None;
    let mut current_departure_time: Option<DateTime<chrono::Utc>> = None;
    let mut current_latest_arrival: Option<DateTime<chrono::Utc>> = None;
    let mut in_this_call = false;
    let mut in_service_departure = false;
    let mut in_service_arrival = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = str::from_utf8(e.local_name().as_ref())
                    .unwrap_or("")
                    .to_string();

                match name.as_str() {
                    "StopEvent" => {
                        current_train_number = None;
                        current_departure_time = None;
                        current_latest_arrival = None;
                    }
                    "ThisCall" => in_this_call = true,
                    "ServiceDeparture" if in_this_call => in_service_departure = true,
                    "ServiceArrival" => in_service_arrival = true,
                    _ => {}
                }

                elem_stack.push(name);
            }

            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.local_name().as_ref()).to_string();

                match name.as_str() {
                    "StopEvent" => {
                        if let (Some(train), Some(dep_time)) =
                            (&current_train_number, &current_departure_time)
                        {
                            if *dep_time <= end_of_today {
                                let has_future_arrival = current_latest_arrival
                                    .map(|latest| latest > now_utc)
                                    .unwrap_or(true);
                                if has_future_arrival {
                                    numbers.insert(train.clone());
                                }
                            }
                        }
                    }
                    "ThisCall" => in_this_call = false,
                    "ServiceDeparture" => in_service_departure = false,
                    "ServiceArrival" => in_service_arrival = false,
                    _ => {}
                }

                elem_stack.pop();
            }

            Ok(Event::Text(e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).trim().to_string();
                handle_text(
                    &text,
                    &elem_stack,
                    &mut current_train_number,
                    &mut current_departure_time,
                    &mut current_latest_arrival,
                    in_service_departure,
                    in_service_arrival,
                );
            }

            Ok(Event::CData(e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).trim().to_string();
                handle_text(
                    &text,
                    &elem_stack,
                    &mut current_train_number,
                    &mut current_departure_time,
                    &mut current_latest_arrival,
                    in_service_departure,
                    in_service_arrival,
                );
            }

            Ok(Event::Eof) => break,

            Err(err) => {
                eprintln!("XML parse error: {}", err);
                break;
            }

            _ => {}
        }

        buf.clear();
    }

    numbers.into_iter().collect()
}

#[cfg(feature = "native-client")]
pub fn fetch_train_numbers(token: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let url = "https://api.opentransportdata.swiss/ojp20";

    let now = chrono::Utc::now();
    let tz = chrono_tz::Europe::Zurich;
    let today_local = now.with_timezone(&tz).date_naive();
    let start_local = tz
        .with_ymd_and_hms(
            today_local.year(),
            today_local.month(),
            today_local.day(),
            4,
            0,
            0,
        )
        .single()
        .ok_or("invalid local start time")?;
    let start_time = start_local
        .with_timezone(&chrono::Utc)
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();

    // Simple XML similar to the Python example; adjust StopPointRef / params as needed.
    let xml_body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <OJP xmlns="http://www.vdv.de/ojp" xmlns:siri="http://www.siri.org.uk/siri" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" version="2.0">
          <OJPRequest>
            <siri:ServiceRequest>
              <siri:ServiceRequestContext>
                <siri:Language>de</siri:Language>
              </siri:ServiceRequestContext>
              <siri:RequestTimestamp>{}</siri:RequestTimestamp>
              <siri:RequestorRef>MyApp</siri:RequestorRef>
              <OJPStopEventRequest>
                <siri:RequestTimestamp>{}</siri:RequestTimestamp>
                <siri:MessageIdentifier>SER_1</siri:MessageIdentifier>
                <Location>
                  <PlaceRef>
                    <siri:StopPointRef>8507000</siri:StopPointRef>
                    <Name><Text>Bern</Text></Name>
                  </PlaceRef>
                  <DepArrTime>{}</DepArrTime>
                </Location>
                <Params>
                  <NumberOfResults>1000</NumberOfResults>
                  <StopEventType>departure</StopEventType>
                  <IncludePreviousCalls>true</IncludePreviousCalls>
                  <IncludeOnwardCalls>true</IncludeOnwardCalls>
                  <UseRealtimeData>full</UseRealtimeData>
                </Params>
              </OJPStopEventRequest>
            </siri:ServiceRequest>
          </OJPRequest>
        </OJP>
        "#,
        start_time, start_time, start_time
    );

    let client = reqwest::blocking::Client::builder().build()?;
    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/xml")
        .body(xml_body)
        .send()?;

    let text = resp.text()?;
    let trains = parse_train_numbers(&text)
        .iter()
        .map(|n| n.parse::<i32>().unwrap())
        .filter(|n| {
            (*n >= 600 && *n <= 649) || (*n >= 800 && *n <= 849) || (*n >= 950 && *n <= 999)
        }) // filter for IC8/81, IC6/61
        .collect::<Vec<i32>>();
    Ok(trains)
}
