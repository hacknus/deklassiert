use chrono::{DateTime, FixedOffset};
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

use quick_xml::events::Event;
// rust
use quick_xml::Reader;
use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};

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
    pub vehicle_identifier: Option<FormationVehicle>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FormationResponse {
    #[serde(default, deserialize_with = "null_to_empty")]
    pub vehicle_journey_type: String,
    pub last_update: DateTime<FixedOffset>,
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
    pub vehicle_properties: Option<VehicleProperties>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VehicleProperties {
    #[serde(default)]
    pub trolley_status: Option<TrolleyStatus>,
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
                Some(TrolleyStatus::Deklassiert) => {
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


pub fn parse_formation_short_string(
    input: &str,
    vehicle_information: &HashMap<u32, (bool, FormationVehicle)>,
) -> Vec<Vehicle> {
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

    for (i, vehicle) in vehicles.iter_mut().enumerate() {
        if false && let Some(coach_number) = vehicle.order_number {
            if let Some((deklassiert, identifier)) = vehicle_information.get(&coach_number) {
                if *deklassiert && !vehicle.status.contains(&StatusFlag::Deklassiert) {
                    vehicle.status.push(StatusFlag::Deklassiert);
                }
                vehicle.vehicle_identifier = Some(identifier.clone());
            }
        } else {
            // could be a locomotive

            for (_number, (deklassiert, identifier)) in vehicle_information.iter() {
                if identifier.position == i as u32 {
                    if *deklassiert && !vehicle.status.contains(&StatusFlag::Deklassiert) {
                        vehicle.status.push(StatusFlag::Deklassiert);
                    }
                    vehicle.vehicle_identifier = Some(identifier.clone());
                }
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
        vehicle_identifier: None,
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
    let base_url = "https://api.opentransportdata.swiss/formation/v2";

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
    in_service_departure: bool,
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
    let today = now_utc.date_naive();
    let tomorrow = today.succ_opt().unwrap();
    let end_of_today = DateTime::<chrono::Utc>::from_naive_utc_and_offset(
        tomorrow.and_hms_opt(4, 0, 0).unwrap(),
        chrono::Utc,
    );

    // per StopEvent state
    let mut current_train_number: Option<String> = None;
    let mut current_departure_time: Option<DateTime<chrono::Utc>> = None;
    let mut in_this_call = false;
    let mut in_service_departure = false;

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
                    }
                    "ThisCall" => in_this_call = true,
                    "ServiceDeparture" if in_this_call => in_service_departure = true,
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
                                numbers.insert(train.clone());
                            }
                        }
                    }
                    "ThisCall" => in_this_call = false,
                    "ServiceDeparture" => in_service_departure = false,
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
                    in_service_departure,
                );
            }

            Ok(Event::CData(e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).trim().to_string();
                handle_text(
                    &text,
                    &elem_stack,
                    &mut current_train_number,
                    &mut current_departure_time,
                    in_service_departure,
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
    let start_time = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();

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
