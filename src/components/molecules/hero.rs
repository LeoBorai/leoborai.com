use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn Hero(title: String, description: String) -> Element {
    rsx! {
        Container { class: "py-10",
            h1 { class: "text-3xl font-bold text-neutral-900 sm:text-4xl dark:text-neutral-100", "{title}" }
            p { class: "mt-4 max-w-2xl text-neutral-600 dark:text-neutral-400", "{description}" }
        }
    }
}
