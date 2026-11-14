use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn NotFoundPage() -> Element {
    rsx! {
        PageFrame {
            div { class: "max-w-2xl",
                h1 { class: "text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100", "Not found" }
                p { class: "mt-6 text-base text-zinc-600 dark:text-zinc-400",
                    "The page you are looking for does not exist."
                }
            }
        }
    }
}
