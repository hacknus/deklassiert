// rust
// File: `src/main.rs`
use dioxus::prelude::*;
use std::{sync::Arc, time::Duration};

#[cfg(feature = "server")]
use once_cell::sync::Lazy;
#[cfg(feature = "server")]
use opentransportdata::parse_formation_json;
#[cfg(feature = "server")]
use std::fs;

mod components;
mod views;

use components::Hero;
use views::{Blog, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")] Home { },
    #[route("/blog/:id")] Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(feature = "server")]
use std::sync::RwLock;

#[cfg(feature = "server")]
static TABS: Lazy<Arc<RwLock<Vec<String>>>> = Lazy::new(|| {
    let data = load_tabs_from_file();
    Arc::new(RwLock::new(data))
});

#[cfg(feature = "server")]
fn load_tabs_from_file() -> Vec<String> {
    let json_data = std::fs::read_to_string("test_data/test_response.json").expect("read json");
    let formation = opentransportdata::parse_formation_json(&json_data).unwrap();

    formation
        .formations_at_scheduled_stops
        .into_iter()
        .map(|s| s.scheduled_stop.stop_point.name)
        .collect()
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
    start_tabs_reload_task();

    dioxus::launch(App);
}

use dioxus::prelude::*;

#[server]
async fn get_tabs() -> Result<Vec<String>, ServerFnError> {
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
