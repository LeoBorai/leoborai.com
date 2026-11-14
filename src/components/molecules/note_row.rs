use dioxus::prelude::*;

use crate::content::*;

#[component]
pub fn NoteRow(note: NoteEntry) -> Element {
    let formatted = note.date.format("%b %-d, %Y").to_string();
    let iso = note.date.to_string();

    rsx! {
        a {
            href: "/notes/{note.slug}",
            class: "group -mx-3 flex items-center gap-6 px-3 py-3 transition-colors hover:bg-neutral-100/60 dark:hover:bg-neutral-900/40",
            time { class: "w-28 shrink-0 font-mono text-xs text-neutral-500", datetime: "{iso}", "{formatted}" }
            span { class: "text-accent min-w-0 truncate text-sm font-medium transition-opacity group-hover:opacity-80",
                "{note.title}"
            }
        }
    }
}
