use dioxus::prelude::*;

use crate::components::*;

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        button {
            r#type: "button",
            "data-theme-toggle": "",
            "aria-label": "Switch theme",
            class: "flex h-9 w-9 cursor-pointer items-center justify-center rounded-lg text-neutral-500 transition-colors hover:bg-neutral-100 hover:text-neutral-900 dark:hover:bg-neutral-800/60 dark:hover:text-neutral-100",
            // sun: shown in dark mode (click -> light)
            Icon {
                view_box: "0 0 24 24",
                class: "hidden h-4 w-4 text-yellow-500 dark:block",
                fill: "none".to_string(),
                stroke: "currentColor".to_string(),
                stroke_width: "1.5".to_string(),
                inner: r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"/>"#,
            }
            // moon: shown in light mode (click -> dark)
            Icon {
                view_box: "0 0 24 24",
                class: "block h-4 w-4 dark:hidden",
                fill: "none".to_string(),
                stroke: "currentColor".to_string(),
                stroke_width: "1.5".to_string(),
                inner: r#"<path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z"/>"#,
            }
        }
    }
}
