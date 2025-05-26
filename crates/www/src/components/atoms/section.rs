use leptos::prelude::*;

#[component]
pub fn Section(
    #[prop(into, optional)] title: String,
    #[prop(into, optional)] description: String,
) -> impl IntoView {
    view! {
        <div id="section-title pb-4">
            <h2 class="text-lg pb-1">{title}</h2>
            <p class="text-sm text-gray-600">{description}</p>
        </div>
    }
}
