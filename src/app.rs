#![allow(non_snake_case)]

use std::fmt::Display;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use leaflet::{Icon, IconOptions};
use wasm_bindgen::prelude::*;

const STYLES_URL: Asset = asset!("/assets/styles.css");
const MAP_URL: Asset = asset!("/assets/map.js");
const LEAFLET_STYLE_URL: Asset = asset!("/assets/leaflet.css");
const LEAFLET_SCRIPT_URL: Asset = asset!("/assets/leaflet.js");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Copy)]
struct Counter {
    value: Signal<i32>,
}

impl Counter {
    fn new() -> Self {
        Self {
            value: use_signal(|| 0),
        }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn decrement(&mut self) {
        self.value -= 1;
    }
}

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn Counter() -> Element {
    let mut counter = Counter::new();

    rsx! {
        div {
            h1 { "Counter: {counter}" }
            button {
                onclick: move |_| counter.increment(),
                "Increment"
            }
            button {
                onclick: move |_| counter.decrement(),
                "Decrement"
            }
        }
    }
}

fn add_map_to_container(id: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let container: web_sys::HtmlElement =
        document.get_element_by_id(id).unwrap().dyn_into().unwrap();
    let map = leaflet::Map::new_with_element(&container, &leaflet::MapOptions::default());
    leaflet::TileLayer::new("https://tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&map);
    map.set_view(&leaflet::LatLng::new(50.03719, 22.00424), 14.0);
    leaflet::Marker::new(&leaflet::LatLng::new(50.03719, 22.00424)).add_to(&map);
}

pub fn App() -> Element {
    use_effect(move || {
        let window = web_sys::window().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            add_map_to_container("map");
        }) as Box<dyn Fn()>);

        // Defer execution so js has time to load
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                100,
            )
            .unwrap();

        closure.forget(); // Prevent closure from being dropped
    });

    rsx! {
        document::Link { rel: "stylesheet", href: STYLES_URL }
        document::Link { rel: "stylesheet", href: LEAFLET_STYLE_URL }
        document::Script { src: LEAFLET_SCRIPT_URL }
        main {
            class: "bg-black text-white w-screen h-screen",

            h1 {
                class: "m-0 w-max",
                "Hello World"
            }
            div { id: "map", class: "w-screen h-screen" }
        }
    }
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }
//
// pub fn App() -> Element {
//     let mut name = use_signal(|| String::new());
//     let mut greet_msg = use_signal(|| String::new());
//
//     let greet = move |_: FormEvent| async move {
//         if name.read().is_empty() {
//             return;
//         }
//
//         let name = name.read();
//         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
//         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//         let new_msg = invoke("greet", args).await.as_string().unwrap();
//         greet_msg.set(new_msg);
//     };
//
//     rsx! {
//         document::Link { rel: "stylesheet", href: STYLES_URL }
//         main {
//             class: "container",
//             h1 { "Welcome to Tauri + Dioxus" }
//
//             div {
//                 class: "row",
//                 a {
//                     href: "https://tauri.app",
//                     target: "_blank",
//                     img {
//                         src: "/tauri.svg",
//                         class: "logo tauri",
//                          alt: "Tauri logo"
//                     }
//                 }
//                 a {
//                     href: "https://dioxuslabs.com/",
//                     target: "_blank",
//                     img {
//                         src: "/dioxus.png",
//                         class: "logo dioxus",
//                         alt: "Dioxus logo"
//                     }
//                 }
//             }
//             p { "Click on the Tauri and Dioxus logos to learn more." }
//
//             form {
//                 class: "row",
//                 onsubmit: greet,
//                 input {
//                     id: "greet-input",
//                     placeholder: "Enter a name...",
//                     value: "{name}",
//                     oninput: move |event| name.set(event.value())
//                 }
//                 button { r#type: "submit", "Greet" }
//             }
//             p { "{greet_msg}" }
//         }
//     }
// }
