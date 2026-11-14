use dioxus::prelude::*;

#[component]
pub fn Container(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "mx-auto w-full max-w-5xl px-6 {class}", {children} }
    }
}
