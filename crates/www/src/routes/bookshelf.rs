use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use reqwest::get;

use proto::BookshelfBook;

use crate::{components::atoms::section::Section, utils::hostname};

#[component]
pub fn Bookshelf() -> impl IntoView {
    let bookshelf: RwSignal<Vec<BookshelfBook>> = RwSignal::new(Vec::default());
    let _ = RenderEffect::new(move |_| {
        spawn_local(async move {
            let Ok(mut url) = hostname() else {
                return;
            };

            url.set_path("/assets/db/bookshelf.json");

            match get(url).await {
                Ok(response) => {
                    if let Ok(data) = response.json::<Vec<BookshelfBook>>().await {
                        bookshelf.set(data);
                    }
                }
                Err(error) => {
                    leptos::logging::error!("Failed to fetch bookshelf {error}");
                }
            }
        });
    });

    view! {
        <Title text=move || "Bookshelf" formatter=|text| format!("{text} — Leo Borai") />
        <div>
            <Section title="Bookshelf" description="Books I have read and recommend." />
            <ul class="py-4">
                <For
                    each=move || bookshelf.get()
                    key=|book| book.isbn_10.clone()
                    children=move |
                        BookshelfBook {
                            title,
                            author,
                            isbn_10,
                            isbn_13,
                            publisher,
                            read_on,
                            review,
                            shopping,
                        }|
                    {
                        let is_online = shopping.online.is_some();
                        let online = shopping.online.clone().unwrap_or_default();

                        view! {
                            <article class="self-start flex flex-col justify-start w-full col-span-4 border-b mb-4">
                                <h3 class="font-semibold pb-2">
                                    {title}{" "}
                                    <span class="block md:inline font-body text-gray-400">
                                        by{" "}{author}
                                    </span>
                                </h3>
                                <p class="pb-2 text-sm text-gray-600">{review}</p>
                                <dl class="text-sm text-gray-600 flex flex-col space-y-2 py-2 text-sm md:space-y-0 md:grid gap-1 md:grid-cols-[repeat(5,20%)]">
                                    <dt class="font-body text-gray-800 font-semibold row-start-1 col-start-1 col-span-1">
                                        "ISBN-10"
                                    </dt>
                                    <dd class="text-xs row-start-2 col-start-1 col-span-1 font-mono">
                                        {isbn_10}
                                    </dd>
                                    <dt class="font-body text-gray-800 font-semibold row-start-1 col-start-2 col-span-1">
                                        "ISBN-13"
                                    </dt>
                                    <dd class="text-xs row-start-2 col-start-2 col-span-1 font-mono">
                                        {isbn_13}
                                    </dd>
                                    <dt class="font-body text-gray-800 font-semibold row-start-1 col-start-3 col-span-1">
                                        "Publisher"
                                    </dt>
                                    <dd class="text-xs row-start-2 col-start-3 col-span-1">
                                        {publisher}
                                    </dd>
                                    <dt class="font-body text-gray-800 font-semibold col-start-4 col-span-1">
                                        "Read On"
                                    </dt>
                                    <dd class="text-xs row-start-2 col-start-4 col-span-1">
                                        {read_on}
                                    </dd>
                                    <dt class="font-body text-gray-800 font-semibold col-start-5 col-span-1">
                                        "Get it"
                                    </dt>
                                    <dd>
                                        <a
                                            class="text-blue-400 hover:text-blue-600 underline"
                                            href=shopping.amazon
                                            target="_blank"
                                        >
                                            "Amazon"
                                        </a>
                                        <br />
                                        <Show when=move || is_online>
                                            <a
                                                class="text-blue-400 hover:text-blue-600 underline"
                                                href=online.clone()
                                                target="_blank"
                                            >
                                                "Online"
                                            </a>
                                        </Show>
                                    </dd>
                                </dl>
                            </article>
                        }
                    }
                />
            </ul>
        </div>
    }
}
