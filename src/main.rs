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

#[cfg(feature = "server")]
use std::sync::RwLock;

#[cfg(feature = "server")]
static TRAINS: Lazy<Arc<RwLock<Vec<FormationResponse>>>> = Lazy::new(|| {
    let data = load_trains_from_dir("test_data");
    Arc::new(RwLock::new(data))
});

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
        loop {
            std::thread::sleep(Duration::from_secs(3600));

            let new_trains = load_trains_from_dir("test_data");
            let mut guard = TRAINS.write().unwrap();
            *guard = new_trains;

            println!("Trains reloaded");
        }
    });
}

fn main() {
    #[cfg(feature = "server")]
    dotenv::dotenv().ok();

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
