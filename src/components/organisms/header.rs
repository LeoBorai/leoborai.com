use dioxus::prelude::*;

use crate::components::*;
use crate::data::{self};

pub fn is_active(path: &str, href: &str) -> bool {
    if href == "/" {
        path == "/"
    } else {
        path.starts_with(href)
    }
}

pub fn dir_label(href: &str) -> String {
    if href == "/" {
        "~".to_string()
    } else {
        format!("~{href}/")
    }
}

#[component]
pub fn Header(path: String) -> Element {
    rsx! {
        header { class: "fixed inset-x-0 top-0 z-50 px-3 pt-3 font-mono sm:px-4",
            div { class: "mx-auto max-w-5xl rounded-2xl border border-neutral-200/80 bg-white/70 shadow-lg shadow-neutral-900/5 backdrop-blur-xl dark:border-neutral-800/80 dark:bg-neutral-950/70 dark:shadow-black/30",
                div { class: "hidden h-14 items-center gap-2 px-3 md:flex",
                    div { class: "flex items-center rounded-xl bg-neutral-100 px-3 py-1.5 dark:bg-neutral-900",
                        Brand {}
                    }
                    nav { class: "flex flex-1 items-center justify-end gap-1", "aria-label": "Primary",
                        for (_label , href) in data::NAV_LINKS {
                            a {
                                key: "{href}",
                                href,
                                "aria-current": if is_active(&path, href) { Some("page") } else { None },
                                class: if is_active(&path, href) {
                                    "rounded-lg px-3 py-1.5 font-mono text-xs whitespace-nowrap transition-colors text-accent bg-accent/10 ring-accent/20 ring-1"
                                } else {
                                    "rounded-lg px-3 py-1.5 font-mono text-xs whitespace-nowrap transition-colors text-neutral-500 hover:bg-neutral-100 hover:text-neutral-900 dark:text-neutral-400 dark:hover:bg-neutral-800/60 dark:hover:text-neutral-100"
                                },
                                "{dir_label(href)}"
                            }
                        }
                    }
                    ThemeToggle {}
                }
                div { class: "flex h-14 items-center justify-between px-4 md:hidden",
                    Brand {}
                    div { class: "flex h-full items-center",
                        ThemeToggle {}
                        button {
                            r#type: "button",
                            id: "menu-toggle",
                            class: "flex h-8 w-8 cursor-pointer items-center justify-center text-neutral-500 hover:text-neutral-900 dark:hover:text-neutral-100",
                            "aria-expanded": "false",
                            "aria-controls": "mobile-menu",
                            "aria-label": "Open menu",
                            Icon {
                                view_box: "0 0 24 24",
                                class: "h-4 w-4",
                                fill: "none".to_string(),
                                stroke: "currentColor".to_string(),
                                stroke_width: "1.5".to_string(),
                                inner: r#"<path data-menu-open stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"/><path data-menu-close hidden stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12"/>"#,
                            }
                        }
                    }
                }
                nav {
                    id: "mobile-menu",
                    "aria-label": "Mobile",
                    class: "hidden flex-col gap-1 border-t border-neutral-200/80 p-2 md:hidden dark:border-neutral-800/80",
                    for (_label , href) in data::NAV_LINKS {
                        a {
                            key: "{href}",
                            href,
                            "aria-current": if is_active(&path, href) { Some("page") } else { None },
                            class: if is_active(&path, href) {
                                "rounded-lg px-4 py-2.5 font-mono text-xs text-accent bg-accent/5"
                            } else {
                                "rounded-lg px-4 py-2.5 font-mono text-xs text-neutral-500 dark:text-neutral-400"
                            },
                            "{dir_label(href)}"
                        }
                    }
                }
            }
        }
    }
}
