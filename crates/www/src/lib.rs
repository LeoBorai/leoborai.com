mod components;
mod routes;
mod utils;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::{Route, Routes, A};
use leptos_router::path;
use routes::projects::Projects;

use self::routes::bookshelf::Bookshelf;
use self::routes::home::Home;
use self::routes::notes::slug::Note;
use self::routes::notes::Notes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <div class="flex flex-col min-h-screen max-w-4xl mx-auto py-6 px-4">
            <header class="flex items-center justify-between">
                <h1 class="font-semibold text-xl">
                    <A href="/">
                        <span class="font-body font-semibold">"Leo Borai"</span>
                    </A>
                </h1>
                <nav class="text-sm">
                    <ul class="flex items-center space-x-4">
                        <li>
                            <A href="/">
                                <span class="font-body font-semibold text-sm">"Home"</span>
                            </A>
                        </li>
                        <li>
                            <A href="/notes">
                                <span class="font-body font-semibold text-sm">"Notes"</span>
                            </A>
                        </li>
                        <li>
                            <A href="/projects">
                                <span class="font-body font-semibold text-sm">"Projects"</span>
                            </A>
                        </li>
                        <li>
                            <A href="/bookshelf">
                                <span class="font-body font-semibold text-sm">"Bookshelf"</span>
                            </A>
                        </li>
                        <li>
                            <a
                                class="font-body font-semibold text-sm"
                                target="_blank"
                                href="https://k8s.leoborai.com"
                            >
                                "K8s Notebook"
                            </a>
                        </li>
                    </ul>
                </nav>
            </header>
            <main class="py-6 min-h-[calc(100vh-130px)]">
                <Title text="Leo Borai" />
                <Routes fallback=move || view! { "Not Found" }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/bookshelf") view=Bookshelf />
                    <Route path=path!("/notes") view=Notes />
                    <Route path=path!("/notes/:slug") view=Note />
                    <Route path=path!("/projects") view=Projects />
                </Routes>
            </main>
            <footer class="flex items-center justify-center p-4">
                <small class="font-body">
                    "Made with 🧉 and ❤️ by Leo Borai © 2017 - 2025"
                </small>
            </footer>
        </div>
    }
}
