use dioxus::prelude::*;

use crate::data::{self};

#[component]
pub fn SkillGroup(label: String, skills: &'static [data::Skill]) -> Element {
    rsx! {
        div { class: "grid gap-2 sm:grid-cols-[8.5rem_minmax(0,1fr)] sm:gap-4",
            dt { class: "text-sm text-neutral-500 dark:text-neutral-400", "{label}" }
            dd { class: "flex flex-wrap gap-x-5 gap-y-2.5",
                for skill in skills {
                    {
                        let class = "group hover:text-accent inline-flex items-center gap-2 text-sm text-neutral-800 transition-colors dark:text-neutral-200";
                        let logo = skill.logo.map(|logo| {
                            rsx! {
                                span {
                                    class: "block size-4 bg-current",
                                    style: "mask:url({logo}) center/contain no-repeat;-webkit-mask:url({logo}) center/contain no-repeat",
                                    "aria-hidden": "true",
                                }
                            }
                        });

                        match skill.href {
                            Some(href) => rsx! {
                                a { key: "{skill.name}", href, target: "_blank", rel: "noopener noreferrer", class, {logo} "{skill.name}" }
                            },
                            None => rsx! {
                                span { key: "{skill.name}", class, {logo} "{skill.name}" }
                            },
                        }
                    }
                }
            }
        }
    }
}
