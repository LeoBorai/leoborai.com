use dioxus::prelude::*;

use crate::components::*;
use crate::content::*;

#[component]
pub fn NotesPage(notes: Vec<NoteEntry>) -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Notes", description: "I write down thoughts and solutions on Software Engineering topics." }
            div { class: "mt-16 sm:mt-20",
                div { class: "md:border-l md:border-zinc-100 md:pl-6 md:dark:border-zinc-700/40",
                    div { class: "flex max-w-3xl flex-col space-y-16",
                        for note in notes {
                            article { key: "{note.slug}", class: "md:grid md:grid-cols-4 md:items-baseline",
                                div { class: "group relative flex flex-col items-start md:col-span-3",
                                    h2 { class: "text-base font-semibold tracking-tight text-zinc-800 dark:text-zinc-100",
                                        div { class: HOVER_BG }
                                        a { href: "/notes/{note.slug}",
                                            span { class: "absolute -inset-x-4 -inset-y-6 z-20 sm:-inset-x-6" }
                                            span { class: "relative z-10", "{note.title}" }
                                        }
                                    }
                                    time {
                                        class: "relative z-10 order-first mb-3 flex items-center pl-3.5 text-sm text-zinc-400 md:hidden dark:text-zinc-500",
                                        datetime: "{note.date}",
                                        span { class: "absolute inset-y-0 left-0 flex items-center", "aria-hidden": "true",
                                            span { class: "h-4 w-0.5 bg-zinc-200 dark:bg-zinc-500" }
                                        }
                                        "{note.date}"
                                    }
                                    p { class: "relative z-10 mt-2 text-sm text-zinc-600 dark:text-zinc-400", "{note.description}" }
                                    div {
                                        "aria-hidden": "true",
                                        class: "text-accent relative z-10 mt-4 flex items-center text-sm font-medium",
                                        "Read note"
                                        Icon { view_box: "0 0 16 16", class: "ml-1 h-4 w-4 stroke-current", fill: "none".to_string(), inner: CHEVRON }
                                    }
                                }
                                time {
                                    class: "relative z-10 order-first mt-1 mb-3 flex items-center text-sm text-zinc-400 max-md:hidden dark:text-zinc-500",
                                    datetime: "{note.date}",
                                    "{note.date}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn date_string(date: chrono::NaiveDate) -> String {
    // same shape as JS `Date.prototype.toDateString()`
    date.format("%a %b %d %Y").to_string()
}

#[component]
pub fn NotePage(note: NoteEntry) -> Element {
    rsx! {
        PageFrame {
            div { class: "xl:relative",
                div { class: "mx-auto max-w-2xl",
                    BackLink { label: "Go back to articles", href: "/notes" }
                    article {
                        header { class: "flex flex-col",
                            h1 { class: "mt-6 text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100",
                                "{note.title}"
                            }
                            time { datetime: "{note.date}", class: "order-first flex items-center text-base text-zinc-400 dark:text-zinc-500",
                                span { class: "h-4 w-0.5 bg-zinc-200 dark:bg-zinc-500" }
                                span { class: "ml-3", "{date_string(note.date)}" }
                            }
                        }
                        div { class: "prose dark:prose-invert mt-8", dangerous_inner_html: note.html }
                    }
                }
            }
        }
    }
}
