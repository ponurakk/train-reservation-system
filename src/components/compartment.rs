use dioxus::prelude::*;

use crate::management::{compartment::CompartmentType, Seat};

#[component]
pub fn Row(row: Vec<Seat>, number: u8) -> Element {
    rsx! {
        div {
            class: "flex gap-3, row",
            for stop in row {
                if stop.is_occupied {
                    div {
                        class: "w-10 h-10 bg-red-900 rounded text-white flex items-center align-middle inline-block",
                        span {
                            "{number}{stop.number}"
                        }
                    }
                } else {
                    div {
                        class: "w-10 h-10 bg-black rounded text-white flex items-center align-middle inline-block",
                        span {
                            "{number}{stop.number}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Compartment(compartment: CompartmentType) -> Element {
    rsx! {
        match compartment {
            CompartmentType::Seat6(compartment) => rsx! {
                for row in compartment.rows {
                    Row { row, number: compartment.number }
                }
            },
            CompartmentType::Seat8(compartment) => rsx! {
                for row in compartment.rows {
                    Row { row, number: compartment.number }
                }
            },
            CompartmentType::OpenSpace(compartment) => rsx! {
                "s"
            }
        }
    }
}
