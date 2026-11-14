use dioxus::prelude::*;

use crate::components::*;
use crate::data::{self};

#[component]
pub fn UsesPage() -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Uses", description: "Hardware & Software that I use to get things done." }
            CardGrid { cards: data::toolbox(), contain: true }
        }
    }
}
