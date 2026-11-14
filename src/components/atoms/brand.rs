use dioxus::prelude::*;

#[component]
pub fn Brand() -> Element {
    rsx! {
        a { href: "/", class: "flex items-center font-mono text-sm font-semibold tracking-tight",
            span { class: "text-accent", "sys@leo" }
            span { class: "text-neutral-400", ":" }
            span { class: "text-neutral-500", "~" }
            span { class: "text-neutral-400", "$" }
        }
    }
}
