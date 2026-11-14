use dioxus::prelude::*;

use crate::components::*;
use crate::data::{self};

#[component]
pub fn ProjectsPage() -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Projects & Contributions", description: "Software I've built or contributed to." }
            CardGrid { cards: data::projects(), contain: false }
        }
    }
}
