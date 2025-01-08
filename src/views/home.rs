use crate::CS_CORE;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use gbrouting::utils::hhmmss_to_duration;

#[component]
pub fn Home() -> Element {
    let stops = use_memo(|| {
        let mut stops = CS_CORE.read().get_stops();
        stops.sort_by(|a, b| a.name.cmp(&b.name));
        stops
    });
    let mut from_stop = use_signal(|| String::new());
    let mut to_stop = use_signal(|| String::new());

    let route = move |_| {
        let r = CS_CORE
            .read()
            .route_optimized_earliest_arrival_with_reconstruction(
                &from_stop.read(),
                &to_stop.read(),
                hhmmss_to_duration("00:00:00").unwrap(),
                None,
            )
            .unwrap()
            .get_result_trip(&CS_CORE.read().connection_scan_data);

        info!("{r:#?}");
    };

    rsx! {
        div {
            class: "w-44 h-40 border border-black rounded p-4 text-center",
            div {
                class: " flex flex-col gap-3",
                select {
                    class: "rounded bg-black p-2 text-black",
                    onchange: move |event| from_stop.set(event.value()),

                    for stop in stops.iter() {
                        option { value: "{stop.id}", "{stop.name}"}
                    }
                }
                select {
                    class: "rounded bg-black p-2 text-black",
                    onchange: move |event| to_stop.set(event.value()),

                    for stop in stops.iter() {
                        option { value: "{stop.id}", "{stop.name}"}
                    }
                }
                button { onclick: route, "Route" }

                "{from_stop} - {to_stop}"
            }
            datalist {
                id: "stops" ,
                for stop in stops.iter() {
                    option { value: "{stop.name}" }
                }
            }
        }
    }
}
