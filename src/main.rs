// rust
// File: `src/main.rs`
use dioxus::prelude::*;
use std::{sync::Arc, time::Duration};

#[cfg(feature = "server")]
use once_cell::sync::Lazy;
#[cfg(feature = "server")]
use std::fs;

use opentransportdata::{parse_formation_json, FormationResponse};

mod components;
mod views;

use views::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")] Home { },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

use chrono::Datelike;
#[cfg(feature = "server")]
use std::sync::RwLock;

#[cfg(feature = "server")]
static TRAINS: Lazy<Arc<RwLock<Vec<FormationResponse>>>> =
    Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

#[cfg(feature = "server")]
fn load_trains_from_dir(dir: &str) -> Vec<FormationResponse> {
    let mut trains = Vec::new();

    let entries = std::fs::read_dir(dir).expect("read dir");

    for entry in entries {
        let path = entry.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let json_data = std::fs::read_to_string(&path).expect("read json");
            let formation = parse_formation_json(&json_data).unwrap();
            trains.push(formation);
        }
    }

    trains
}

#[cfg(feature = "server")]
pub fn start_tabs_reload_task() {
    std::thread::spawn(|| {

        dotenv::dotenv().ok();

        let formation_token = std::env::var("FORMATION_TOKEN").expect("TOKEN not set");
        let ojp_token = std::env::var("OJP_TOKEN").expect("set OJP_TOKEN env var");


        loop {
            // get all IC8/81, IC6/61 before the end of day
            let trains = opentransportdata::fetch_train_numbers(&ojp_token);

            let now_utc = chrono::Utc::now();
            let today = now_utc.date_naive();
            let year = today.year();
            let month = today.month();
            let day = today.day();

            let mut new_trains = vec![];

            println!("Loaded trains: {:?}", trains);

            if let Ok(trains) = trains {
                for train in trains {
                    println!("Loading formation for train {}", train);
                    let formation = opentransportdata::get_train_formation(
                        train,
                        year,
                        month,
                        day,
                        &formation_token,
                    )
                    .unwrap();
                    new_trains.push(formation);
                    let mut guard = TRAINS.write().unwrap();
                    *guard = new_trains.clone();

                    // wait because of max 5 requests per minute
                    std::thread::sleep(Duration::from_secs(12));
                }
            }

            std::thread::sleep(Duration::from_secs(3600));

            println!("Trains reloaded");
        }
    });
}

fn main() {
    #[cfg(feature = "server")]
    start_tabs_reload_task();

    dioxus::launch(App);
}

#[server]
async fn get_trains() -> Result<Vec<FormationResponse>, ServerFnError> {
    Ok(TRAINS.read().unwrap().clone())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
