use leptos::{component, view, IntoView};

const CAREER_START_YEAR: i32 = 2017;
const RUST_START_YEAR: i32 = 2019;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="space-y-2">
            <section class="text-gray-600">
                <article class="text-sm">
                    <p>"Software Developer"</p>
                    <p>"South America (UTC-3)"</p>
                </article>
            </section>
            <section>
                <h3 class="font-semibold pb-2 text-lg text-gray-800">"Bio"</h3>
                <article class="text-sm text-gray-600">
                    <h3 class="underline my-2">"Web Development Experience"</h3>
                    <p>
                        "Started to work as a Web Developer with ReactJS and C# in "{CAREER_START_YEAR}", then moved into other similar positions"
                        " using technologies like NodeJS, NextJS, Go, Svelte/Kit and EmberJS."
                    </p>
                    <h3 class="underline my-2">"Rust Experience & Systems Programming"</h3>
                    <p>
                        "In  "{RUST_START_YEAR}" I started learning Rust with interest in the Systems Programming field."
                        " Since then I have been mainly focused in Rust and its ecosystem, I keep it up with Rust by using it on"
                        " my daily job and personal projects."
                    </p>
                </article>
            </section>
        </div>
    }
}
