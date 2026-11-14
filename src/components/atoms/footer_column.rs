use dioxus::prelude::*;

#[component]
pub fn FooterColumn(title: String) -> Element {
    rsx! {
        h3 { class: "font-mono text-xs tracking-wider text-neutral-500 uppercase", "{title}" }
    }
}
