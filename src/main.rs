use dioxus::prelude::*;

use components::Navbar;
use dioxus_logger::tracing::{info, Level};
use gbrouting::csa::routing::ConnectionScanCore;
use views::{Home, Train};

mod components;
mod management;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/train")]
    Train {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// const MAP_URL: Asset = asset!("/assets/map.js");
const LEAFLET_STYLE_URL: Asset = asset!("/assets/leaflet.css");
const LEAFLET_SCRIPT_URL: Asset = asset!("/assets/leaflet.js");

static CS_CORE: GlobalSignal<ConnectionScanCore> = Global::new(ConnectionScanCore::default);

fn main() {
    dioxus_logger::init(Level::INFO).expect("Failed to init logger");
    dioxus::launch(App);
}

async fn load_gtfs() -> ConnectionScanCore {
    info!("Loading gtfs");
    let gtfs = gbrouting::gtfs::load("./gtfs/pkpic.zip").unwrap();

    info!("Loading cs_data");
    let cs_data = gbrouting::csa::gtfs::parse_gtfs(
        &gtfs,
        gbrouting::exports::Local::now().naive_local().date(),
        true,
        1000.0,
        0.6,
        false,
    )
    .unwrap();

    ConnectionScanCore::new(&cs_data)
}

#[component]
fn App() -> Element {
    let mut loaded = use_signal(|| false);
    spawn(async move {
        if loaded() {
            return;
        };

        if let Ok(cs_core) = tokio::spawn(load_gtfs()).await {
            info!("Creating cs_core");
            *CS_CORE.write() = cs_core;
            *loaded.write() = true;
        }
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Link { rel: "stylesheet", href: LEAFLET_STYLE_URL }
        document::Script { src: LEAFLET_SCRIPT_URL }

        if loaded() {
            Router::<Route> {}
        } else {
            div {
                class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                p {
                    "Loading gtfs data"
                }
            }
        }
    }
}
