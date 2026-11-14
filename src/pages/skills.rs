use dioxus::prelude::*;

use crate::components::*;
use crate::content::*;

#[component]
pub fn SkillsPage(skills: Vec<SkillEntry>) -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Skills", description: "AI Agent Skills I've built to extend how coding agents work." }
            div { class: "mt-16 sm:mt-20",
                div { class: "md:border-l md:border-zinc-100 md:pl-6 md:dark:border-zinc-700/40",
                    div { class: "flex max-w-3xl flex-col space-y-16",
                        for skill in skills {
                            article { key: "{skill.slug}", class: "flex flex-col items-start",
                                div { class: "group relative flex flex-col items-start",
                                    h2 { class: "text-base font-semibold tracking-tight text-zinc-800 dark:text-zinc-100",
                                        div { class: HOVER_BG }
                                        a { href: "/skills/{skill.slug}",
                                            span { class: "absolute -inset-x-4 -inset-y-6 z-20 sm:-inset-x-6" }
                                            span { class: "relative z-10", "{skill.name}" }
                                        }
                                    }
                                    p { class: "relative z-10 mt-2 text-sm text-zinc-600 dark:text-zinc-400", "{skill.description}" }
                                    div {
                                        "aria-hidden": "true",
                                        class: "text-accent relative z-10 mt-4 flex items-center text-sm font-medium",
                                        "Read skill"
                                        Icon { view_box: "0 0 16 16", class: "ml-1 h-4 w-4 stroke-current", fill: "none".to_string(), inner: CHEVRON }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SkillPage(skill: SkillEntry) -> Element {
    rsx! {
        PageFrame {
            div { class: "xl:relative",
                div { class: "mx-auto max-w-2xl",
                    BackLink { label: "Go back to skills", href: "/skills" }
                    article {
                        header { class: "flex flex-col",
                            div { class: "flex items-start justify-between gap-4",
                                h1 { class: "mt-6 text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100",
                                    "{skill.name}"
                                }
                                div { class: "mt-6 flex-none",
                                    // leading newline is dropped by HTML parsers, so content survives verbatim
                                    textarea { id: "copy-source", hidden: true, readonly: true, "\n{skill.markdown}" }
                                    button {
                                        r#type: "button",
                                        "data-copy-source": "copy-source",
                                        class: "group hover:text-accent dark:hover:text-accent flex cursor-pointer items-center gap-2 bg-white/90 px-3 py-2 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm transition dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10 dark:hover:ring-white/20",
                                        Icon {
                                            view_box: "0 0 16 16",
                                            class: "h-4 w-4 stroke-current",
                                            fill: "none".to_string(),
                                            inner: r#"<path d="M5.75 5.75h5.5a1 1 0 0 1 1 1v5.5a1 1 0 0 1-1 1h-5.5a1 1 0 0 1-1-1v-5.5a1 1 0 0 1 1-1Z" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M3.75 9.75h-1a1 1 0 0 1-1-1v-5.5a1 1 0 0 1 1-1h5.5a1 1 0 0 1 1 1v1" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>"#,
                                        }
                                        span { "data-copy-label": "", "Copy as MD" }
                                    }
                                }
                            }
                            p { class: "order-first flex items-center text-base text-zinc-400 dark:text-zinc-500",
                                "{skill.description}"
                            }
                        }
                        div { class: "prose dark:prose-invert mt-8", dangerous_inner_html: skill.html }
                    }
                }
            }
        }
    }
}
