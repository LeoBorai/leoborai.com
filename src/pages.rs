//! Pages. `Root` renders the shared chrome around one `Page` variant.

mod home;
mod not_found;
mod notes;
mod projects;
mod skills;
mod uses;

use dioxus::prelude::*;

use crate::components::*;
use crate::content::{NoteEntry, SkillEntry};

use home::HomePage;
use not_found::NotFoundPage;
use notes::{NotePage, NotesPage};
use projects::ProjectsPage;
use skills::{SkillPage, SkillsPage};
use uses::UsesPage;

#[derive(Clone, PartialEq)]
pub enum Page {
    Home { notes: Vec<NoteEntry> },
    Notes { notes: Vec<NoteEntry> },
    Note { note: NoteEntry },
    Skills { skills: Vec<SkillEntry> },
    Skill { skill: SkillEntry },
    Projects,
    Uses,
    NotFound,
}

#[component]
pub fn Root(path: String, page: Page, year: i32) -> Element {
    rsx! {
        div {
            class: "dotted-background fixed inset-0 -z-10",
            "aria-hidden": "true",
        }
        div {
            class: "pointer-events-none fixed inset-0 z-0 hidden [mask-image:linear-gradient(#000,transparent_min(95vh,900px))] xl:block",
            "aria-hidden": "true",
            div { class: "absolute inset-y-0 left-0 w-[calc(50%-40rem+1px)] border-r border-neutral-200 dark:border-neutral-800" }
            div { class: "absolute inset-y-0 right-0 w-[calc(50%-40rem+1px)] border-l border-neutral-200 dark:border-neutral-800" }
        }
        canvas {
            id: "pixel-field",
            "aria-hidden": "true",
            class: "pointer-events-none fixed inset-x-0 top-0 z-0 block h-[85vh] w-full [mask-image:linear-gradient(to_bottom,#000_65%,transparent)] select-none",
        }
        canvas {
            id: "rail-glow",
            "aria-hidden": "true",
            class: "pointer-events-none fixed inset-0 z-0 hidden h-screen w-full [mask-image:linear-gradient(#000,transparent_min(95vh,900px))] select-none xl:block",
        }
        div { class: "relative min-h-screen overflow-x-clip text-neutral-900 dark:text-neutral-100",
            Header { path: path.clone() }
            main { class: "relative z-10 pt-20",
                match page {
                    Page::Home { notes } => rsx! { HomePage { notes } },
                    Page::Notes { notes } => rsx! { NotesPage { notes } },
                    Page::Note { note } => rsx! { NotePage { note } },
                    Page::Skills { skills } => rsx! { SkillsPage { skills } },
                    Page::Skill { skill } => rsx! { SkillPage { skill } },
                    Page::Projects => rsx! { ProjectsPage {} },
                    Page::Uses => rsx! { UsesPage {} },
                    Page::NotFound => rsx! { NotFoundPage {} },
                }
            }
            Footer { year }
        }
    }
}
