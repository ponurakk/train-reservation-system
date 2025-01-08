use crate::{components::Wagon, management};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[component]
pub fn Train() -> Element {
    let train = management::Train::generate_random_train();

    rsx! {
        div {
            for wagon in train.wagons {
                Wagon { wagon }
            }
            br {}
            br {}
        }
    }
}
