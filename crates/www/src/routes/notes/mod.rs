pub mod slug;

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use leptos_router::components::A;
use proto::{NotesIndex, RichNoteMetadata};
use reqwest::get;

use crate::{components::atoms::section::Section, utils::hostname};

#[component]
pub fn Notes() -> impl IntoView {
    let notes_index: RwSignal<Vec<RichNoteMetadata>> = RwSignal::new(Vec::default());
    let _ = RenderEffect::new(move |_| {
        spawn_local(async move {
            let Ok(mut url) = hostname() else {
                return;
            };

            url.set_path("/assets/notes_index.json");

            match get(url).await {
                Ok(response) => {
                    if let Ok(idx) = response.json::<NotesIndex>().await {
                        notes_index.set(idx.0.clone());
                    }
                }
                Err(error) => {
                    leptos::logging::error!("Failed to fetch notes {error}");
                }
            }
        });
    });

    view! {
        <Title text=move || "Notes" formatter=|text| format!("{text} — Leo Borai") />
        <div>
            <Section
                title="Notes"
                description="Notes on Software Development journeys. Here I keep notes that others might find useful, and for my future self to serve as reference."
            />
            <ul class="py-4">
                <For
                    each=move || notes_index.get()
                    key=|note| note.slug.clone()
                    children=move |RichNoteMetadata { meta, slug }| {
                        view! {
                            <article class="self-start flex flex-col justify-start w-full col-span-4 border-b mb-4">
                                <h3 class="hover:underline">
                                    <A exact=true href=format!("/notes/{slug}")>
                                        <span class="font-body font-semibold">{meta.title}</span>
                                    </A>
                                </h3>
                                <p class="text-gray-600 py-2 text-sm truncate">
                                    {meta.description}
                                </p>
                                <ul class="flex justify-start items-start flex-wrap gap-2 py-2">
                                    <For
                                        each=move || meta.categories.clone()
                                        key=|cat| cat.clone()
                                        children=move |cat: String| {
                                            view! {
                                                <li class="text-xs bg-gray-100 text-gray-800 uppercase rounded-md py-0.5 px-2">
                                                    {cat}
                                                </li>
                                            }
                                        }
                                    />
                                </ul>
                                <div class="flex">
                                    <span class="flex items-center">
                                        <time
                                            class="py-2 text-xs uppercase"
                                            datetime=meta.date.to_string()
                                        >
                                            {meta.date.to_string()}
                                        </time>
                                    </span>
                                </div>
                            </article>
                        }
                    }
                />
            </ul>
        </div>
    }
}
