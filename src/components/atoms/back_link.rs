use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn BackLink(label: String, href: String) -> Element {
    rsx! {
        a {
            "aria-label": "{label}",
            class: "group mb-8 flex h-10 w-10 items-center justify-center bg-white shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 transition lg:absolute lg:-left-5 lg:-mt-2 lg:mb-0 xl:-top-1.5 xl:left-0 xl:mt-0 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0 dark:ring-white/10 dark:hover:border-zinc-700 dark:hover:ring-white/20",
            href,
            Icon {
                view_box: "0 0 16 16",
                class: "h-4 w-4 stroke-zinc-500 transition group-hover:stroke-zinc-700 dark:stroke-zinc-500 dark:group-hover:stroke-zinc-400",
                fill: "none".to_string(),
                inner: r#"<path d="M7.25 11.25 3.75 8m0 0 3.5-3.25M3.75 8h8.5" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>"#,
            }
        }
    }
}
