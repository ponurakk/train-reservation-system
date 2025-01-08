use dioxus::prelude::*;

use crate::components::Compartment;
use crate::management;
use crate::management::compartment::CompartmentType;

#[component]
pub fn Wagon(wagon: management::Wagon) -> Element {
    rsx! {
        div {
            class: "flex gap-3 wagon border rounded p-3",
            for compartment in wagon.compartments {
                div {
                    class: "flex flex-col gap-1 compartment",
                    Compartment { compartment }
                }
            }
        }
    }
}
