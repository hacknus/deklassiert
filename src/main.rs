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
static TABS: Lazy<Arc<RwLock<FormationResponse>>> = Lazy::new(|| {
    let data = load_tabs_from_file();
    Arc::new(RwLock::new(data))
});

#[cfg(feature = "server")]
fn load_tabs_from_file() -> FormationResponse {
    // todo replace string with vehicles at stop plus stop data

    let json_data = std::fs::read_to_string("test_data/test_response.json").expect("read json");
    let formation = parse_formation_json(&json_data).unwrap();

    // formation
    //     .formations_at_scheduled_stops
    //     .into_iter()
    //     .map(|s| s.scheduled_stop.stop_point.name)
    //     .collect();

    formation
}

#[cfg(feature = "server")]
pub fn start_tabs_reload_task() {
    std::thread::spawn(|| {
        loop {
            std::thread::sleep(Duration::from_secs(3600));

            let new_tabs = load_tabs_from_file();
            let mut guard = TABS.write().unwrap();
            *guard = new_tabs;

            println!("Tabs reloaded");
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
async fn get_train() -> Result<FormationResponse, ServerFnError> {
    Ok(TABS.read().unwrap().clone())
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
