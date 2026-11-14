use dioxus::prelude::*;

use crate::components::*;
use crate::data::{self, *};

#[component]
pub fn Footer(year: i32) -> Element {
    rsx! {
        footer { class: "relative mt-12 w-full border-t border-neutral-200 bg-neutral-50 dark:border-neutral-800 dark:bg-neutral-950",
            div { class: "mx-auto max-w-7xl border-x border-neutral-200 bg-neutral-100/40 px-6 py-12 dark:border-neutral-800 dark:bg-neutral-950/40",
                div { class: "grid grid-cols-1 gap-12 md:grid-cols-3",
                    div { class: "space-y-3",
                        FooterColumn { title: "About" }
                        p { class: "text-sm text-neutral-500 dark:text-neutral-400",
                            "Doing Software Development since 2017. Experience in Web Development, Systems Programming and now learning AI Engineering."
                        }
                    }
                    div { class: "space-y-3",
                        FooterColumn { title: "Sitemap" }
                        nav { "aria-label": "Footer", class: "flex flex-col space-y-2",
                            for (label , href) in data::NAV_LINKS {
                                a {
                                    key: "{href}",
                                    href,
                                    class: "hover:text-accent text-sm text-neutral-500 transition-colors dark:text-neutral-400",
                                    "{label}"
                                }
                            }
                        }
                    }
                    div { class: "space-y-3",
                        FooterColumn { title: "Connect" }
                        nav { "aria-label": "Social", class: "flex flex-col space-y-2",
                            for link in data::social_links() {
                                a {
                                    key: "{link.label}",
                                    href: "{link.href}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "hover:text-accent inline-flex items-center gap-2 text-sm text-neutral-500 transition-colors dark:text-neutral-400",
                                    SocialIcon { link: link.clone(), class: "h-4 w-4" }
                                    "{link.label}"
                                }
                            }
                        }
                    }
                }
                div { class: "mt-12 flex items-center justify-center border-t border-neutral-200 pt-6 dark:border-neutral-800",
                    p { class: "text-center text-xs text-neutral-500", "© {year} {SITE_NAME}. All rights reserved." }
                }
            }
        }
    }
}
