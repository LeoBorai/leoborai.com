use dioxus::prelude::*;

use crate::components::*;
use crate::content::*;
use crate::data::{self, *};

#[component]
pub fn HomePage(notes: Vec<NoteEntry>) -> Element {
    rsx! {
        Container { class: "py-10",
            div { class: "flex flex-col items-center text-center",
                h1 { class: "mb-2 text-3xl font-bold sm:text-4xl", "{SITE_NAME}" }
                p { class: "text-accent mb-2 text-xl sm:text-2xl", "{SITE_ROLE}" }
                p {
                    class: "mb-6 h-4 font-mono text-xs text-neutral-500 tabular-nums",
                    "aria-label": "Local time",
                    "Europe · "
                    span { id: "clock" }
                }
                ul { class: "flex flex-wrap justify-center gap-2",
                    for link in data::social_links() {
                        li { key: "{link.label}",
                            a {
                                href: "{link.href}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                "aria-label": "{link.label}",
                                class: "hover:text-accent hover:border-accent/50 inline-flex border border-neutral-200 p-2 text-neutral-500 transition-all dark:border-neutral-800",
                                SocialIcon { link: link.clone(), class: "size-5" }
                            }
                        }
                    }
                }
            }
        }
        Container {
            div { class: "grid gap-x-12 lg:grid-cols-2",
                Section { id: "notes".to_string(), label: "Notes", href: "/notes".to_string(), href_label: "View all notes".to_string(),
                    div { class: "divide-y divide-neutral-200 dark:divide-neutral-800",
                        for note in notes {
                            NoteRow { key: "{note.slug}", note: note.clone() }
                        }
                    }
                }
                Section { id: "skills".to_string(), label: "Stack & Skills",
                    dl { class: "space-y-4",
                        for (label , skills) in SKILL_GROUPS {
                            SkillGroup { key: "{label}", label: label.to_string(), skills: *skills }
                        }
                    }
                }
            }
        }
    }
}
