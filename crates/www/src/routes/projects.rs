use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use proto::Project;
use reqwest::get;

use crate::components::atoms::icons::{GitHub, Globe};
use crate::components::atoms::section::Section;
use crate::utils::hostname;

#[component]
pub fn Projects() -> impl IntoView {
    let projects: RwSignal<Vec<Project>> = RwSignal::new(Vec::default());
    let _ = RenderEffect::new(move |_| {
        spawn_local(async move {
            let Ok(mut url) = hostname() else {
                return;
            };

            url.set_path("/assets/db/projects.json");

            match get(url).await {
                Ok(response) => {
                    if let Ok(data) = response.json::<Vec<Project>>().await {
                        projects.set(data);
                    }
                }
                Err(error) => {
                    leptos::logging::error!("Failed to fetch projects {error}");
                }
            }
        });
    });

    view! {
        <Title text=move || "Projects" formatter=|text| format!("{text} — Leo Borai") />
        <div>
            <Section
                title="Projects"
                description="Open-Source projects that I have been part of or that I maintain."
            />
            <ul class="py-4 grid gap-4 grid-cols-[100%] md:grid-cols-[repeat(2,50%)]">
                <For
                    each=move || projects.get()
                    key=|p| p.title.clone()
                    children=move |Project { title, repo_url, extract, website, tags, langs }| {
                        let has_website = website.is_some();
                        let website_url = website.clone().unwrap_or_default();
                        let render_website = move || {
                            if has_website {
                                return 
                                view! {
                                    <a
                                        class="ml-2 hover:underline gap-2 text-gray-600 hover:text-gray-800 h-4 w-4"
                                        href=website_url
                                        target="_blank"
                                    >
                                        <Globe class="inline h-4 w-4" />
                                        <small class="pl-2 text-xs">"Website"</small>
                                    </a>
                                }
                                    .into_any();
                            }
                            view! {};
                            ().into_any()
                        };

                        view! {
                            <li class="border border-gray-200 rounded-md p-2">
                                <h3 class="pb-2 text-sm">{title}</h3>
                                <p class="text-xs text-gray-600 pb-2">{extract}</p>
                                <div class="flex justify-between items-center pb-2">
                                    <div>
                                        <a
                                            class="hover:underline gap-2 text-gray-600 hover:text-gray-800 h-4 w-4"
                                            href=repo_url
                                            target="_blank"
                                        >
                                            <GitHub class="inline h-4 w-4" />
                                            <small class="pl-2 text-xs">"Source"</small>
                                        </a>
                                        {render_website()}
                                    </div>
                                    <ul class="pb-2 flex gap-2">
                                        <For
                                            each=move || langs.clone()
                                            key=|l| l.clone()
                                            children=move |lang| {
                                                let lc_lang = lang.to_ascii_lowercase();

                                                view! {
                                                    <li class=format!(
                                                        "lang-{} text-xs bg-gray-100 text-gray-800 uppercase rounded-md py-0.5 px-2",
                                                        lc_lang,
                                                    )>{lang}</li>
                                                }
                                            }
                                        />
                                    </ul>
                                </div>
                                <ul class="flex gap-2">
                                    <For
                                        each=move || tags.clone()
                                        key=|t| t.clone()
                                        children=move |tag| {
                                            view! {
                                                <li class="text-xs bg-gray-100 text-gray-800 uppercase rounded-md py-0.5 px-2">
                                                    {tag}
                                                </li>
                                            }
                                        }
                                    />
                                </ul>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
