use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn PageFrame(children: Element) -> Element {
    rsx! {
        div { class: PAGE_OUTER,
            div { class: "mx-auto w-full max-w-7xl lg:px-8",
                div { class: "relative px-4 sm:px-8 lg:px-12",
                    div { class: "mx-auto max-w-2xl lg:max-w-5xl", {children} }
                }
            }
        }
    }
}
