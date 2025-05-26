use leptos::{prelude::*, task::spawn_local};
use leptos_meta::Title;
use leptos_router::{components::A, hooks::use_params_map};
use reqwest::get;
use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;
use yaml_front_matter::YamlFrontMatter;

use crate::utils::hostname;

struct Markdown {
    html: String,
    headings: Vec<String>,
}

#[derive(Clone, Deserialize)]
struct Metadata {
    title: String,
    description: String,
    date: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = highlightAll, js_namespace = hljs)]
    fn highlight_all();
}

#[component]
pub fn Note() -> impl IntoView {
    let metadata: RwSignal<Option<Metadata>> = RwSignal::new(None);
    let note_md: RwSignal<Option<String>> = RwSignal::new(None);
    let headings: RwSignal<Vec<String>> = RwSignal::new(Vec::default());
    let _ = RenderEffect::new(move |_| {
        let params_map = use_params_map();

        spawn_local(async move {
            // FIXME: Check slugs against list of notes
            if let Some(slug) = params_map.get_untracked().get("slug") {
                if slug.is_empty() {
                    return;
                }

                let Ok(mut url) = hostname() else {
                    note_md.set(Some("Error: Failed to retrieve hostname".to_string()));
                    return;
                };

                url.set_path(&format!("/assets/notes/{slug}.md"));

                match get(url).await {
                    Ok(response) => {
                        if let Ok(text) = response.text().await {
                            if let Ok(document) = YamlFrontMatter::parse::<Metadata>(&text) {
                                metadata.set(Some(document.metadata));

                                let md = parse_md_into_html(&document.content);
                                note_md.set(Some(md.html));
                                headings.set(md.headings);

                                highlight_all();
                            }
                        } else {
                            note_md.set(None);
                        }
                    }
                    Err(error) => {
                        note_md.set(Some(format!("Error: {}", error)));
                    }
                }
            }
        });
    });

    view! {
        <Title
            text=move || metadata.get().map(|meta| meta.title).unwrap_or_default()
            formatter=|text| format!("{text} — Leo Borai")
        />
        <section id="note-container" class="pb-10">
            <article id="note-header" class="pb-8">
                <h1 class="text-3xl font-semibold py-2">
                    {move || metadata.get().map(|meta| meta.title).unwrap_or_default()}
                </h1>
                <p class="text-gray-600 text-sm">
                    {{ move || metadata.get().map(|meta| meta.description).unwrap_or_default() }}
                </p>
                <time class="text-sm text-gray-600">
                    {move || metadata.get().map(|meta| meta.date).unwrap_or_default()}
                </time>
            </article>
            <div class="md:relative md:grid md:gap-4 md:grid-cols-[900px,200px]">
                <aside id="note-toc" class="md:row-start-1 md:col-start-2 md:col-span-1 md:w-full">
                    <strong class="text-lg font-semibold">"Table of Contents"</strong>
                    <ul class="space-y-2">
                        <For
                            each=move || headings.get()
                            key=|heading| heading.clone()
                            children=move |heading: String| {
                                view! {
                                    <li class="text-sm text-gray-400 hover:text-gray-600">
                                        <A exact=true href=format!("#{}", heading)>
                                            {heading}
                                        </A>
                                    </li>
                                }
                            }
                        />
                    </ul>
                </aside>
                <div
                    id="note-content"
                    class="md:row-start-1 md:col-start-1 md:col-span-1 md:w-fill"
                    inner_html=move || note_md.get().unwrap_or_default()
                ></div>
            </div>
            <div class="group grid grid-cols-[100px,auto] gap-4 border-t border-gray-400 pt-4">
                <figure class="grayscale group-hover:grayscale-0 rounded-full overflow-hidden flex justify-center items-center h-[100px] w-[100px]">
                    <img src="/assets/images/whoami.jpg" alt="whoami" height="100" width="100" />
                </figure>
                <article>
                    <h1 class="text-lg text-gray-800">"Leo Borai"</h1>
                    <div class="text-sm space-y-2 text-gray-600">
                        <p>
                            "Hi there! I'm a "<u>"Rust Software Engineer"</u>
                            " with 8 years of experience in Systems and Web Programming using Rust & TypeScript."
                        </p>
                        <p>
                            "I'm passionate about Open-Source and enjoy reading books, working out & playing videogames."
                        </p>
                        <p>
                            "I've had the opportunity to work with companies like InfinyOn, GOintegro & Teleperformance."
                        </p>
                    </div>
                </article>
            </div>
        </section>
    }
}

fn parse_md_into_html(md: &str) -> Markdown {
    use pulldown_cmark::{Event, Parser, Tag};

    let mut text: Vec<String> = Vec::new();
    let mut headings: Vec<String> = Vec::new();
    let mut state = String::new();
    let mut is_heading = false;
    let mut is_codeblock = false;

    for event in Parser::new(md) {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Paragraph => state.push_str(r#"<p class="my-2 py-2">"#),
                    Tag::Heading { level, .. } => {
                        state.push_str(&format!(r#"<{}>"#, level.to_string().to_ascii_lowercase()));
                        is_heading = true;
                    }
                    Tag::CodeBlock(_) => {
                        state.push_str("<pre><code>");
                        is_codeblock = true;
                    }
                    Tag::BlockQuote(_) => {
                        state.push_str(r#"<blockquote class="bg-yellow-100 p-1 my-2">"#);
                    }
                    Tag::List(mb_items) => {
                        if mb_items.is_some() {
                            state.push_str(r#"<ol class="list-decimal space-y-2">"#)
                        } else {
                            state.push_str(r#"<ul class="list-disc space-y-2">"#)
                        }
                    }
                    Tag::Item => state.push_str("<li>"),
                    Tag::Image { dest_url, .. } => {
                        state.push_str(&format!(r#"<img src="{}" />"#, dest_url));
                    }
                    Tag::Link {
                        dest_url, title, ..
                    } => {
                        state.push_str(&format!(
                            r#"<a class="text-blue-600 underline" target="_blank" href="{}">{}"#,
                            dest_url, title
                        ));
                    }
                    _ => {} // Tag::HtmlBlock => todo!(),
                            // Tag::FootnoteDefinition(cow_str) => todo!(),
                            // Tag::DefinitionList => todo!(),
                            // Tag::DefinitionListTitle => todo!(),
                            // Tag::DefinitionListDefinition => todo!(),
                            // Tag::Table(vec) => todo!(),
                            // Tag::TableHead => todo!(),
                            // Tag::TableRow => todo!(),
                            // Tag::TableCell => todo!(),
                            // Tag::Emphasis => todo!(),
                            // Tag::Strong => todo!(),
                            // Tag::Strikethrough => todo!(),
                            // Tag::MetadataBlock(metadata_block_kind) => todo!(),
                }
            }
            Event::End(tag_end) => {
                match tag_end {
                    pulldown_cmark::TagEnd::Paragraph => state.push_str("</p>"),
                    pulldown_cmark::TagEnd::CodeBlock => {
                        state.push_str("</code></pre>");
                        is_codeblock = false;
                    }
                    pulldown_cmark::TagEnd::Heading(level) => {
                        state.push_str(&format!("</{}>", level.to_string().to_ascii_lowercase()));
                        is_heading = false;
                    }
                    pulldown_cmark::TagEnd::BlockQuote(_) => {
                        state.push_str("</blockquote>");
                    }
                    pulldown_cmark::TagEnd::List(is_ordered) => {
                        leptos::logging::log!("{:?}", is_ordered);
                        if is_ordered {
                            state.push_str("</ol>")
                        } else {
                            state.push_str("</ul>")
                        }
                    }
                    pulldown_cmark::TagEnd::Item => state.push_str("</li>"),
                    pulldown_cmark::TagEnd::Link => {
                        state.push_str("</a>");
                    }
                    _ => {} // pulldown_cmark::TagEnd::HtmlBlock => todo!(),
                            // pulldown_cmark::TagEnd::FootnoteDefinition => todo!(),
                            // pulldown_cmark::TagEnd::DefinitionList => todo!(),
                            // pulldown_cmark::TagEnd::DefinitionListTitle => todo!(),
                            // pulldown_cmark::TagEnd::DefinitionListDefinition => todo!(),
                            // pulldown_cmark::TagEnd::Table => todo!(),
                            // pulldown_cmark::TagEnd::TableHead => todo!(),
                            // pulldown_cmark::TagEnd::TableRow => todo!(),
                            // pulldown_cmark::TagEnd::TableCell => todo!(),
                            // pulldown_cmark::TagEnd::Emphasis => todo!(),
                            // pulldown_cmark::TagEnd::Strong => todo!(),
                            // pulldown_cmark::TagEnd::Strikethrough => todo!(),
                            // pulldown_cmark::TagEnd::Link => todo!(),
                            // pulldown_cmark::TagEnd::Image => todo!(),
                            // pulldown_cmark::TagEnd::MetadataBlock(metadata_block_kind) => todo!(),
                }

                text.push(state.clone());
                state.clear();
            }
            Event::Text(cow_str) => {
                if !cow_str.is_empty() {
                    if is_heading {
                        headings.push(cow_str.to_string());
                    }

                    if is_codeblock {
                        state.push_str(
                            &cow_str
                                .to_string()
                                .replace("&", "&amp;")
                                .replace("<", "&lt;")
                                .replace(">", "&gt;")
                                .replace("\"", "&quot;")
                                .replace("'", "&#039;"),
                        );
                    } else {
                        state.push_str(&cow_str);
                    }
                }
            }
            Event::Code(cow_str) => {
                state.push_str(&format!(r#"<code class="bg-gray-50 font-semibold text-sm p-0.5 rounded-md text-rose-600 border border-rose-600">{}</code>"#, cow_str));
            }
            _ => {} // Event::InlineMath(cow_str) => todo!(),
                    // Event::DisplayMath(cow_str) => todo!(),
                    // Event::Html(cow_str) => todo!(),
                    // Event::InlineHtml(cow_str) => todo!(),
                    // Event::FootnoteReference(cow_str) => todo!(),
                    // Event::SoftBreak => todo!(),
                    // Event::HardBreak => todo!(),
                    // Event::Rule => todo!(),
                    // Event::TaskListMarker(_) => todo!(),
        }
    }

    leptos::logging::log!("{:#?}", text);

    Markdown {
        html: text.join(""),
        headings,
    }
}
