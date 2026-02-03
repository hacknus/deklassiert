use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormationResponse {
    pub vehicle_journey_type: String,
    pub last_update: DateTime<FixedOffset>,
    pub journey_meta_information: JourneyMetaInformation,
    pub train_meta_information: TrainMetaInformation,
    pub formations_at_scheduled_stops: Vec<FormationAtScheduledStop>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JourneyMetaInformation {
    pub operation_date: String,

    #[serde(rename = "SJYID")]
    pub sjyid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrainMetaInformation {
    pub train_number: u32,
    pub to_code: String,
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
    pub stop_type: String,
    pub stop_time: StopTime,
    pub track: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StopPoint {
    pub uic: u32,
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

pub fn parse_formation_json(json: &str) -> Result<FormationResponse, serde_json::Error> {
    serde_json::from_str::<FormationResponse>(json)
}

fn parse_vehicle_type(s: &str) -> VehicleType {
    match s {
        "1" => VehicleType::FirstClass,
        "2" => VehicleType::SecondClass,
        "12" => VehicleType::FirstAndSecondClass,
        "FA" => VehicleType::FamilyCar,
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

pub fn parse_formation_short_string(input: &str) -> Vec<Vehicle> {
    let mut vehicles = Vec::new();
    let mut buf = String::new();
    let mut current_sector: Option<char> = None;

    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '@' => {
                if let Some(sec) = chars.next() {
                    current_sector = Some(sec);
                }
            }
            '[' | ']' | ',' => {
                if let Some(vehicle) = parse_vehicle(buf.trim(), current_sector) {
                    vehicles.push(vehicle);
                }
                buf.clear();
            }
            _ => buf.push(ch),
        }
    }

    if let Some(vehicle) = parse_vehicle(buf.trim(), current_sector) {
        vehicles.push(vehicle);
    }

    // check for deklassiert vehicles and set the flag
    for vehicle in vehicles.iter_mut() {
        if vehicle.vehicle_type != VehicleType::Locomotive && !vehicle.offers.contains(&Offer::LowFloor) && !vehicle.offers.contains(&Offer::BikeHooks) && !vehicle.offers.contains(&Offer::BikeReserved) {
            // this is an EW IV or EuroCity coach. If there are no bike mounts, this is likely a deklassiert vehicle
            if !vehicle.status.contains(&StatusFlag::Closed) && vehicle.vehicle_type != VehicleType::FirstClass && vehicle.vehicle_type != VehicleType::FirstAndSecondClass && vehicle.vehicle_type != VehicleType::DiningFirstClass {
                vehicle.status.push(StatusFlag::Deklassiert);
            }
        }
    }

    vehicles
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
    })
}

#[cfg(feature = "native-client")]
pub fn get_train_formation(
    train_id: i32,
    year: i32,
    month: i32,
    day: i32,
    token: &str,
) -> Result<FormationResponse, String> {
    let base_url = "https://api.opentransportdata.swiss/formation/v2";

    let url = format!(
        "{}/formations_stop_based?evu=SBBP&operationDate={}-{}-{}&trainNumber={}",
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
