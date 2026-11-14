use dioxus::prelude::*;

#[component]
pub fn Section(
    #[props(default)] id: Option<String>,
    label: String,
    #[props(default)] href: Option<String>,
    #[props(default)] href_label: Option<String>,
    children: Element,
) -> Element {
    let href_label = href_label.unwrap_or_else(|| "View all".to_string());

    rsx! {
        section { id, class: "scroll-mt-20 py-6",
            div { class: "mb-4 flex flex-wrap items-center justify-between gap-x-4 gap-y-1",
                h2 { class: "font-mono text-[11px] tracking-[0.2em] text-neutral-500 uppercase", "{label}" }
                if let Some(href) = href {
                    a {
                        href,
                        class: "hover:text-accent font-mono text-[10px] tracking-wider whitespace-nowrap text-neutral-500 uppercase transition-colors",
                        "{href_label} →"
                    }
                }
            }
            {children}
        }
    }
}
