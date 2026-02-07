use dioxus::prelude::*;
use std::{sync::Arc, time::Duration};

mod components;
mod views;

use views::{All, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/all")]
    All {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css",
    AssetOptions::css().with_static_head(true)
);
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css",
    AssetOptions::css().with_static_head(true)
);
// const SBB_WEB_ROMAN_WOFF2: Asset = asset!("/assets/Grafikdateien-SBB-Font/WEB/SBBWeb-Roman.woff2");
// const SBB_WEB_ROMAN_WOFF: Asset = asset!("/assets/Grafikdateien-SBB-Font/WEB/SBBWeb-Roman.woff");
// const SBB_WEB_ITALIC_WOFF2: Asset = asset!("/assets/Grafikdateien-SBB-Font/WEB/SBBWeb-Italic.woff2");
// const SBB_WEB_ITALIC_WOFF: Asset = asset!("/assets/Grafikdateien-SBB-Font/WEB/SBBWeb-Italic.woff");
// const SBB_WEB_BOLD_WOFF2: Asset = asset!("/assets/Grafikdateien-SBB-Font/WEB/SBBWeb-Bold.woff2");
// const SBB_WEB_BOLD_WOFF: Asset = asset!("/assets/Grafikdateien-SBB-Font/WEB/SBBWeb-Bold.woff");

use chrono::Datelike;

#[cfg(feature = "server")]
use once_cell::sync::Lazy;
#[cfg(feature = "server")]
use std::sync::RwLock;

use opentransportdata::{parse_formation_json, FormationResponse};

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

        let is_train_finished = |train: &FormationResponse, now: chrono::DateTime<chrono::Utc>| {
            let last_time = train
                .formations_at_scheduled_stops
                .iter()
                .rev()
                .find_map(|stop| {
                    stop.scheduled_stop
                        .stop_time
                        .departure_time
                        .or(stop.scheduled_stop.stop_time.arrival_time)
                });
            match last_time {
                Some(time) => time.with_timezone(&chrono::Utc) < now,
                None => false,
            }
        };

        loop {
            let trains = opentransportdata::fetch_train_numbers(&ojp_token);

            let now_utc = chrono::Utc::now();
            let today = now_utc.date_naive();
            let year = today.year();
            let month = today.month();
            let day = today.day();

            let mut train_map = {
                let guard = TRAINS.read().unwrap();
                guard
                    .iter()
                    .cloned()
                    .map(|t| (t.train_meta_information.train_number, t))
                    .collect::<std::collections::BTreeMap<_, _>>()
            };

            println!("Loaded trains: {:?}", trains);

            match trains {
                Ok(trains) => {
                    'train_loop: for train in trains {
                        'load_train: loop {
                            println!("Loading formation for train {}", train);
                            match opentransportdata::get_train_formation(
                                train,
                                year,
                                month,
                                day,
                                &formation_token,
                            ) {
                                Err(e) => {
                                    if e.contains("Too Many Requests") {
                                        println!("Rate limit hit, sleeping for 60 seconds");
                                        std::thread::sleep(Duration::from_secs(60));
                                        continue 'load_train;
                                    } else if e.contains("Forbidden") {
                                        println!("Forbidden for train {}, skipping", train);
                                        std::thread::sleep(Duration::from_secs(12));
                                        continue 'train_loop ;
                                    } else if e.contains("Bad Request") {
                                        println!("Bad request for train {}, skipping", train);
                                        std::thread::sleep(Duration::from_secs(12));
                                        continue 'train_loop;
                                    }
                                    println!("Error loading formation for train {}: {}", train, e);
                                    break 'load_train;
                                }
                                Ok(formation) => {
                                    train_map.insert(formation.train_meta_information.train_number, formation);
                                    let mut guard = TRAINS.write().unwrap();
                                    *guard = train_map.values().cloned().collect();
                                    break 'load_train;
                                }
                            }
                        }

                        std::thread::sleep(Duration::from_secs(12));
                    }

                    // Remove trains that have already passed.
                    train_map.retain(|_, formation| !is_train_finished(formation, now_utc));

                    let mut guard = TRAINS.write().unwrap();
                    *guard = train_map.values().cloned().collect();
                }
                Err(e) => {
                    println!("Error fetching train numbers: {}", e);
                }
            }

            std::thread::sleep(Duration::from_secs(3600));
            println!("Trains reloaded");
        }
    });
}

#[server]
async fn get_trains() -> Result<Vec<FormationResponse>, ServerFnError> {
    Ok(TRAINS.read().unwrap().clone())
}

#[component]
fn App() -> Element {
//     let font_css = format!(
//         r#"
// @font-face {{
//     font-family: "SBB Web";
//     src:
//         url("{SBB_WEB_ROMAN_WOFF2}") format("woff2"),
//         url("{SBB_WEB_ROMAN_WOFF}") format("woff");
//     font-weight: 400;
//     font-style: normal;
//     font-display: swap;
// }}
// @font-face {{
//     font-family: "SBB Web";
//     src:
//         url("{SBB_WEB_ITALIC_WOFF2}") format("woff2"),
//         url("{SBB_WEB_ITALIC_WOFF}") format("woff");
//     font-weight: 400;
//     font-style: italic;
//     font-display: swap;
// }}
// @font-face {{
//     font-family: "SBB Web";
//     src:
//         url("{SBB_WEB_BOLD_WOFF2}") format("woff2"),
//         url("{SBB_WEB_BOLD_WOFF}") format("woff");
//     font-weight: 700;
//     font-style: normal;
//     font-display: swap;
// }}
// "#
//     );

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // document::Style { "{font_css}" }

        Router::<Route> {}
    }
}

//
// ================= SERVER MAIN =================
//
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use std::net::SocketAddr;
    dotenv::dotenv().ok();

    start_tabs_reload_task();

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8081);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{}", addr);

    let router = Router::new().serve_dioxus_application(ServeConfig::default(), App);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

//
// ================= WASM MAIN =================
//
#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}
