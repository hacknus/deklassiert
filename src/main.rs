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

        loop {
            let trains = opentransportdata::fetch_train_numbers(&ojp_token);

            let now_utc = chrono::Utc::now();
            let today = now_utc.date_naive();
            let year = today.year();
            let month = today.month();
            let day = today.day();

            // TODO: remove old trains and update the ones that have changed instead of reloading all formations every time
            let mut new_trains = vec![];

            println!("Loaded trains: {:?}", trains);

            match trains {
                Ok(trains) => {
                    for train in trains {
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
                                    }
                                    println!("Error loading formation for train {}: {}", train, e);
                                }
                                Ok(formation) => {
                                    new_trains.push(formation);

                                    let mut guard = TRAINS.write().unwrap();
                                    *guard = new_trains.clone();
                                    break 'load_train;
                                }
                            }
                        }

                        std::thread::sleep(Duration::from_secs(12));
                    }
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
