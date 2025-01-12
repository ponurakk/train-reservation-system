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
                        class: "w-12 h-10 m-[1px] bg-red-900 rounded text-white flex items-center align-middle inline-block border-2 border-black",
                        if stop.number % 2 == 1{
                            div {
                                class: "ml-1 w-10 h-10 bg-red-900 rounded text-white flex items-center align-middle inline-block border-2 border-black",
                                p {
                                    class: "text-l m-[25%]",
                                    "{number}{stop.number}"
                                }
                            }
                        } else {
                            div {
                                class: "w-10 h-10 bg-red-900 rounded text-white flex items-center align-middle inline-block border-2 border-black",
                                p {
                                    class: "text-l m-[25%]",
                                    "{number}{stop.number}"
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "w-12 h-10 m-[1px] bg-gray-800 rounded text-white flex items-center align-middle inline-block border-2 border-black",
                        if stop.number % 2 == 1{
                            div {
                                class: "ml-1 w-10 h-10 bg-gray-800 rounded text-white flex items-center align-middle inline-block border-2 border-black",
                                span {
                                    class: "text-l m-[25%]",
                                    "{number}{stop.number}"
                                }
                            }
                        } else {
                            div {
                                class: "w-10 h-10 bg-gray-800 rounded text-white flex items-center align-middle inline-block border-2 border-black",
                                span {
                                    class: "text-l m-[25%]",
                                    "{number}{stop.number}"
                                }
                            }
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
                div{
                    class: "p-1 rounded border-2 border-gray-800",
                    for row in compartment.rows {
                        Row { row, number: compartment.number }
                    }
                }
            },
            CompartmentType::Seat8(compartment) => rsx! {
                div{
                    class: "p-1 rounded border-2 border-gray-800",
                    for row in compartment.rows {
                        Row { row, number: compartment.number }
                    }
                }
            },
            CompartmentType::OpenSpace(compartment) => rsx! {
                "s"
            }
        }
    }
}
