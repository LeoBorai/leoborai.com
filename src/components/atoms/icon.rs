use dioxus::prelude::*;

use crate::data::SocialLink;

/// Raw SVG markup inside an `<svg>` element with shared attributes.
#[component]
pub fn Icon(
    view_box: String,
    class: String,
    #[props(default)] fill: Option<String>,
    #[props(default)] stroke: Option<String>,
    #[props(default)] stroke_width: Option<String>,
    inner: String,
) -> Element {
    rsx! {
        svg {
            view_box,
            class,
            fill,
            stroke,
            stroke_width,
            "aria-hidden": "true",
            dangerous_inner_html: inner,
        }
    }
}

fn social_icon_markup(link: &SocialLink) -> String {
    let rule = if link.evenodd { "evenodd" } else { "nonzero" };
    let mut inner = format!("<path d=\"{}\" fill-rule=\"{rule}\"/>", link.path);

    for layer in &link.layers {
        inner.push_str(&format!("<path d=\"{layer}\"/>"));
    }

    inner
}

#[component]
pub fn SocialIcon(link: SocialLink, class: String) -> Element {
    rsx! {
        Icon {
            view_box: link.view_box.clone(),
            class,
            fill: "currentColor".to_string(),
            inner: social_icon_markup(&link),
        }
    }
}
