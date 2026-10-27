//! Dioxus components. Rendered to static HTML with `dioxus-ssr`.

use chrono::NaiveDate;
use dioxus::prelude::*;

use crate::content::{NoteEntry, SkillEntry};
use crate::data::{self, Card, SITE_NAME, SITE_ROLE, SKILL_GROUPS, SocialLink};

#[derive(Clone, PartialEq)]
pub enum Page {
    Home { notes: Vec<NoteEntry> },
    Notes { notes: Vec<NoteEntry> },
    Note { note: NoteEntry },
    Skills { skills: Vec<SkillEntry> },
    Skill { skill: SkillEntry },
    Projects,
    Uses,
    NotFound,
}

#[component]
pub fn Root(path: String, page: Page, year: i32) -> Element {
    rsx! {
        div {
            class: "dotted-background fixed inset-0 -z-10",
            "aria-hidden": "true",
        }
        div {
            class: "pointer-events-none fixed inset-0 z-0 hidden [mask-image:linear-gradient(#000,transparent_min(95vh,900px))] xl:block",
            "aria-hidden": "true",
            div { class: "absolute inset-y-0 left-0 w-[calc(50%-40rem+1px)] border-r border-neutral-200 dark:border-neutral-800" }
            div { class: "absolute inset-y-0 right-0 w-[calc(50%-40rem+1px)] border-l border-neutral-200 dark:border-neutral-800" }
        }
        canvas {
            id: "pixel-field",
            "aria-hidden": "true",
            class: "pointer-events-none fixed inset-x-0 top-0 z-0 block h-[85vh] w-full [mask-image:linear-gradient(to_bottom,#000_65%,transparent)] select-none",
        }
        canvas {
            id: "rail-glow",
            "aria-hidden": "true",
            class: "pointer-events-none fixed inset-0 z-0 hidden h-screen w-full [mask-image:linear-gradient(#000,transparent_min(95vh,900px))] select-none xl:block",
        }
        div { class: "relative min-h-screen text-neutral-900 dark:text-neutral-100",
            Header { path: path.clone() }
            main { class: "relative z-10 pt-20",
                match page {
                    Page::Home { notes } => rsx! { HomePage { notes } },
                    Page::Notes { notes } => rsx! { NotesPage { notes } },
                    Page::Note { note } => rsx! { NotePage { note } },
                    Page::Skills { skills } => rsx! { SkillsPage { skills } },
                    Page::Skill { skill } => rsx! { SkillPage { skill } },
                    Page::Projects => rsx! { ProjectsPage {} },
                    Page::Uses => rsx! { UsesPage {} },
                    Page::NotFound => rsx! { NotFoundPage {} },
                }
            }
            Footer { year }
        }
    }
}

// ───────────────────────────── primitives ─────────────────────────────

#[component]
fn Container(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "mx-auto w-full max-w-5xl px-6 {class}", {children} }
    }
}

#[component]
fn Hero(title: String, description: String) -> Element {
    rsx! {
        Container { class: "py-10",
            h1 { class: "text-3xl font-bold text-neutral-900 sm:text-4xl dark:text-neutral-100", "{title}" }
            p { class: "mt-4 max-w-2xl text-neutral-600 dark:text-neutral-400", "{description}" }
        }
    }
}

#[component]
fn Section(
    #[props(default)] id: Option<String>,
    label: String,
    #[props(default)] href: Option<String>,
    #[props(default)] href_label: Option<String>,
    children: Element,
) -> Element {
    let href_label = href_label.unwrap_or_else(|| "View all".to_string());

    rsx! {
        section { id, class: "scroll-mt-20 py-6",
            div { class: "mb-4 flex items-center justify-between",
                h2 { class: "font-mono text-[11px] tracking-[0.2em] text-neutral-500 uppercase", "{label}" }
                if let Some(href) = href {
                    a {
                        href,
                        class: "hover:text-accent font-mono text-[10px] tracking-wider text-neutral-500 uppercase transition-colors",
                        "{href_label} →"
                    }
                }
            }
            {children}
        }
    }
}

/// Raw SVG markup inside an `<svg>` element with shared attributes.
#[component]
fn Icon(
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
fn SocialIcon(link: SocialLink, class: String) -> Element {
    rsx! {
        Icon {
            view_box: link.view_box.clone(),
            class,
            fill: "currentColor".to_string(),
            inner: social_icon_markup(&link),
        }
    }
}

// ───────────────────────────── chrome ─────────────────────────────

fn is_active(path: &str, href: &str) -> bool {
    if href == "/" {
        path == "/"
    } else {
        path.starts_with(href)
    }
}

fn dir_label(href: &str) -> String {
    if href == "/" {
        "~".to_string()
    } else {
        format!("~{href}/")
    }
}

#[component]
fn Brand() -> Element {
    rsx! {
        a { href: "/", class: "flex items-center font-mono text-sm font-semibold tracking-tight",
            span { class: "text-accent", "sys@leo" }
            span { class: "text-neutral-400", ":" }
            span { class: "text-neutral-500", "~" }
            span { class: "text-neutral-400", "$" }
        }
    }
}

#[component]
fn ThemeToggle() -> Element {
    rsx! {
        button {
            r#type: "button",
            "data-theme-toggle": "",
            "aria-label": "Switch theme",
            class: "flex h-9 w-9 cursor-pointer items-center justify-center rounded-lg text-neutral-500 transition-colors hover:bg-neutral-100 hover:text-neutral-900 dark:hover:bg-neutral-800/60 dark:hover:text-neutral-100",
            // sun: shown in dark mode (click -> light)
            Icon {
                view_box: "0 0 24 24",
                class: "hidden h-4 w-4 text-yellow-500 dark:block",
                fill: "none".to_string(),
                stroke: "currentColor".to_string(),
                stroke_width: "1.5".to_string(),
                inner: r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"/>"#,
            }
            // moon: shown in light mode (click -> dark)
            Icon {
                view_box: "0 0 24 24",
                class: "block h-4 w-4 dark:hidden",
                fill: "none".to_string(),
                stroke: "currentColor".to_string(),
                stroke_width: "1.5".to_string(),
                inner: r#"<path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z"/>"#,
            }
        }
    }
}

#[component]
fn Header(path: String) -> Element {
    rsx! {
        header { class: "fixed inset-x-0 top-0 z-50 px-3 pt-3 font-mono sm:px-4",
            div { class: "mx-auto max-w-5xl rounded-2xl border border-neutral-200/80 bg-white/70 shadow-lg shadow-neutral-900/5 backdrop-blur-xl dark:border-neutral-800/80 dark:bg-neutral-950/70 dark:shadow-black/30",
                div { class: "hidden h-14 items-center gap-2 px-3 md:flex",
                    div { class: "flex items-center rounded-xl bg-neutral-100 px-3 py-1.5 dark:bg-neutral-900",
                        Brand {}
                    }
                    nav { class: "flex flex-1 items-center justify-end gap-1", "aria-label": "Primary",
                        for (_label , href) in data::NAV_LINKS {
                            a {
                                key: "{href}",
                                href,
                                "aria-current": if is_active(&path, href) { Some("page") } else { None },
                                class: if is_active(&path, href) {
                                    "rounded-lg px-3 py-1.5 font-mono text-xs whitespace-nowrap transition-colors text-accent bg-accent/10 ring-accent/20 ring-1"
                                } else {
                                    "rounded-lg px-3 py-1.5 font-mono text-xs whitespace-nowrap transition-colors text-neutral-500 hover:bg-neutral-100 hover:text-neutral-900 dark:text-neutral-400 dark:hover:bg-neutral-800/60 dark:hover:text-neutral-100"
                                },
                                "{dir_label(href)}"
                            }
                        }
                    }
                    ThemeToggle {}
                }
                div { class: "flex h-14 items-center justify-between px-4 md:hidden",
                    Brand {}
                    div { class: "flex h-full items-center",
                        ThemeToggle {}
                        button {
                            r#type: "button",
                            id: "menu-toggle",
                            class: "flex h-8 w-8 cursor-pointer items-center justify-center text-neutral-500 hover:text-neutral-900 dark:hover:text-neutral-100",
                            "aria-expanded": "false",
                            "aria-controls": "mobile-menu",
                            "aria-label": "Open menu",
                            Icon {
                                view_box: "0 0 24 24",
                                class: "h-4 w-4",
                                fill: "none".to_string(),
                                stroke: "currentColor".to_string(),
                                stroke_width: "1.5".to_string(),
                                inner: r#"<path data-menu-open stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"/><path data-menu-close hidden stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12"/>"#,
                            }
                        }
                    }
                }
                nav {
                    id: "mobile-menu",
                    "aria-label": "Mobile",
                    class: "hidden flex-col gap-1 border-t border-neutral-200/80 p-2 md:hidden dark:border-neutral-800/80",
                    for (_label , href) in data::NAV_LINKS {
                        a {
                            key: "{href}",
                            href,
                            "aria-current": if is_active(&path, href) { Some("page") } else { None },
                            class: if is_active(&path, href) {
                                "rounded-lg px-4 py-2.5 font-mono text-xs text-accent bg-accent/5"
                            } else {
                                "rounded-lg px-4 py-2.5 font-mono text-xs text-neutral-500 dark:text-neutral-400"
                            },
                            "{dir_label(href)}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FooterColumn(title: String) -> Element {
    rsx! {
        h3 { class: "font-mono text-xs tracking-wider text-neutral-500 uppercase", "{title}" }
    }
}

#[component]
fn Footer(year: i32) -> Element {
    rsx! {
        footer { class: "relative mt-12 w-full border-t border-neutral-200 bg-neutral-50 dark:border-neutral-800 dark:bg-neutral-950",
            div { class: "mx-auto max-w-7xl border-x border-neutral-200 bg-neutral-100/40 px-6 py-12 dark:border-neutral-800 dark:bg-neutral-950/40",
                div { class: "grid grid-cols-1 gap-12 md:grid-cols-3",
                    div { class: "space-y-3",
                        FooterColumn { title: "About" }
                        p { class: "text-sm text-neutral-500 dark:text-neutral-400",
                            "{SITE_NAME} — {SITE_ROLE}. Web Development, Systems Programming and AI Engineering."
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

// ───────────────────────────── home ─────────────────────────────

#[component]
fn NoteRow(note: NoteEntry) -> Element {
    let formatted = note.date.format("%b %-d, %Y").to_string();
    let iso = note.date.to_string();

    rsx! {
        a {
            href: "/notes/{note.slug}",
            class: "group -mx-3 flex items-center gap-6 px-3 py-3 transition-colors hover:bg-neutral-100/60 dark:hover:bg-neutral-900/40",
            time { class: "w-28 shrink-0 font-mono text-xs text-neutral-500", datetime: "{iso}", "{formatted}" }
            span { class: "text-accent truncate text-sm font-medium transition-opacity group-hover:opacity-80",
                "{note.title}"
            }
        }
    }
}

#[component]
fn SkillGroup(label: String, skills: &'static [data::Skill]) -> Element {
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

#[component]
fn HomePage(notes: Vec<NoteEntry>) -> Element {
    rsx! {
        Container { class: "py-10",
            div { class: "flex flex-col gap-8 md:flex-row md:items-center md:justify-between md:gap-16",
                div { class: "min-w-0 flex-1",
                    h1 { class: "mb-2 text-3xl font-bold sm:text-4xl", "{SITE_NAME}" }
                    p { class: "text-accent mb-2 text-xl sm:text-2xl", "{SITE_ROLE}" }
                    p {
                        class: "mb-6 h-4 font-mono text-xs text-neutral-500 tabular-nums",
                        "aria-label": "Local time",
                        "Europe · "
                        span { id: "clock" }
                    }
                    ul { class: "flex flex-wrap gap-2",
                        for link in data::social_links() {
                            li { key: "{link.label}",
                                a {
                                    href: "{link.href}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "aria-label": "{link.label}",
                                    class: "hover:text-accent hover:border-accent/50 inline-flex border border-neutral-200 p-2 text-neutral-500 transition-all dark:border-neutral-800",
                                    SocialIcon { link: link.clone(), class: "size-5" }
                                }
                            }
                        }
                    }
                }
                div { class: "min-w-0 flex-1 md:max-w-md",
                    h2 { class: "mb-3 font-mono text-[11px] tracking-[0.2em] text-neutral-500 uppercase", "About" }
                    p { class: "leading-relaxed text-neutral-600 dark:text-neutral-400",
                        "Doing Software Development since 2017. Experience in Web Development, Systems Programming and now learning AI Engineering."
                    }
                }
            }
        }
        Container {
            div { class: "grid gap-x-12 lg:grid-cols-2",
                Section { id: "notes".to_string(), label: "Notes", href: "/notes".to_string(), href_label: "View all notes".to_string(),
                    div { class: "divide-y divide-neutral-200 dark:divide-neutral-800",
                        for note in notes {
                            NoteRow { key: "{note.slug}", note: note.clone() }
                        }
                    }
                }
                Section { id: "skills".to_string(), label: "Stack & Skills",
                    dl { class: "space-y-4",
                        for (label , skills) in SKILL_GROUPS {
                            SkillGroup { key: "{label}", label: label.to_string(), skills: *skills }
                        }
                    }
                }
            }
        }
    }
}

// ───────────────────────────── lists ─────────────────────────────

const PAGE_OUTER: &str = "mt-9 sm:px-8";

#[component]
fn PageFrame(children: Element) -> Element {
    rsx! {
        div { class: PAGE_OUTER,
            div { class: "mx-auto w-full max-w-7xl lg:px-8",
                div { class: "relative px-4 sm:px-8 lg:px-12",
                    div { class: "mx-auto max-w-2xl lg:max-w-5xl", {children} }
                }
            }
        }
    }
}

const CHEVRON: &str = r#"<path d="M6.75 5.75 9.25 8l-2.5 2.25" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>"#;
const HOVER_BG: &str = "absolute -inset-x-4 -inset-y-6 z-0 scale-95 bg-zinc-50 opacity-0 transition group-hover:scale-100 group-hover:opacity-100 sm:-inset-x-6 dark:bg-zinc-800/50";

#[component]
fn NotesPage(notes: Vec<NoteEntry>) -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Notes", description: "I write down thoughts and solutions on Software Engineering topics." }
            div { class: "mt-16 sm:mt-20",
                div { class: "md:border-l md:border-zinc-100 md:pl-6 md:dark:border-zinc-700/40",
                    div { class: "flex max-w-3xl flex-col space-y-16",
                        for note in notes {
                            article { key: "{note.slug}", class: "md:grid md:grid-cols-4 md:items-baseline",
                                div { class: "group relative flex flex-col items-start md:col-span-3",
                                    h2 { class: "text-base font-semibold tracking-tight text-zinc-800 dark:text-zinc-100",
                                        div { class: HOVER_BG }
                                        a { href: "/notes/{note.slug}",
                                            span { class: "absolute -inset-x-4 -inset-y-6 z-20 sm:-inset-x-6" }
                                            span { class: "relative z-10", "{note.title}" }
                                        }
                                    }
                                    time {
                                        class: "relative z-10 order-first mb-3 flex items-center pl-3.5 text-sm text-zinc-400 md:hidden dark:text-zinc-500",
                                        datetime: "{note.date}",
                                        span { class: "absolute inset-y-0 left-0 flex items-center", "aria-hidden": "true",
                                            span { class: "h-4 w-0.5 bg-zinc-200 dark:bg-zinc-500" }
                                        }
                                        "{note.date}"
                                    }
                                    p { class: "relative z-10 mt-2 text-sm text-zinc-600 dark:text-zinc-400", "{note.description}" }
                                    div {
                                        "aria-hidden": "true",
                                        class: "text-accent relative z-10 mt-4 flex items-center text-sm font-medium",
                                        "Read note"
                                        Icon { view_box: "0 0 16 16", class: "ml-1 h-4 w-4 stroke-current", fill: "none".to_string(), inner: CHEVRON }
                                    }
                                }
                                time {
                                    class: "relative z-10 order-first mt-1 mb-3 flex items-center text-sm text-zinc-400 max-md:hidden dark:text-zinc-500",
                                    datetime: "{note.date}",
                                    "{note.date}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillsPage(skills: Vec<SkillEntry>) -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Skills", description: "AI Agent Skills I've built to extend how coding agents work." }
            div { class: "mt-16 sm:mt-20",
                div { class: "md:border-l md:border-zinc-100 md:pl-6 md:dark:border-zinc-700/40",
                    div { class: "flex max-w-3xl flex-col space-y-16",
                        for skill in skills {
                            article { key: "{skill.slug}", class: "flex flex-col items-start",
                                div { class: "group relative flex flex-col items-start",
                                    h2 { class: "text-base font-semibold tracking-tight text-zinc-800 dark:text-zinc-100",
                                        div { class: HOVER_BG }
                                        a { href: "/skills/{skill.slug}",
                                            span { class: "absolute -inset-x-4 -inset-y-6 z-20 sm:-inset-x-6" }
                                            span { class: "relative z-10", "{skill.name}" }
                                        }
                                    }
                                    p { class: "relative z-10 mt-2 text-sm text-zinc-600 dark:text-zinc-400", "{skill.description}" }
                                    div {
                                        "aria-hidden": "true",
                                        class: "text-accent relative z-10 mt-4 flex items-center text-sm font-medium",
                                        "Read skill"
                                        Icon { view_box: "0 0 16 16", class: "ml-1 h-4 w-4 stroke-current", fill: "none".to_string(), inner: CHEVRON }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ───────────────────────────── detail pages ─────────────────────────────

#[component]
fn BackLink(label: String, href: String) -> Element {
    rsx! {
        a {
            "aria-label": "{label}",
            class: "group mb-8 flex h-10 w-10 items-center justify-center bg-white shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 transition lg:absolute lg:-left-5 lg:-mt-2 lg:mb-0 xl:-top-1.5 xl:left-0 xl:mt-0 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0 dark:ring-white/10 dark:hover:border-zinc-700 dark:hover:ring-white/20",
            href,
            Icon {
                view_box: "0 0 16 16",
                class: "h-4 w-4 stroke-zinc-500 transition group-hover:stroke-zinc-700 dark:stroke-zinc-500 dark:group-hover:stroke-zinc-400",
                fill: "none".to_string(),
                inner: r#"<path d="M7.25 11.25 3.75 8m0 0 3.5-3.25M3.75 8h8.5" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>"#,
            }
        }
    }
}

fn date_string(date: NaiveDate) -> String {
    // same shape as JS `Date.prototype.toDateString()`
    date.format("%a %b %d %Y").to_string()
}

#[component]
fn NotePage(note: NoteEntry) -> Element {
    rsx! {
        PageFrame {
            div { class: "xl:relative",
                div { class: "mx-auto max-w-2xl",
                    BackLink { label: "Go back to articles", href: "/notes" }
                    article {
                        header { class: "flex flex-col",
                            h1 { class: "mt-6 text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100",
                                "{note.title}"
                            }
                            time { datetime: "{note.date}", class: "order-first flex items-center text-base text-zinc-400 dark:text-zinc-500",
                                span { class: "h-4 w-0.5 bg-zinc-200 dark:bg-zinc-500" }
                                span { class: "ml-3", "{date_string(note.date)}" }
                            }
                        }
                        div { class: "prose dark:prose-invert mt-8", dangerous_inner_html: note.html }
                    }
                }
            }
        }
    }
}

#[component]
fn SkillPage(skill: SkillEntry) -> Element {
    rsx! {
        PageFrame {
            div { class: "xl:relative",
                div { class: "mx-auto max-w-2xl",
                    BackLink { label: "Go back to skills", href: "/skills" }
                    article {
                        header { class: "flex flex-col",
                            div { class: "flex items-start justify-between gap-4",
                                h1 { class: "mt-6 text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100",
                                    "{skill.name}"
                                }
                                div { class: "mt-6 flex-none",
                                    // leading newline is dropped by HTML parsers, so content survives verbatim
                                    textarea { id: "copy-source", hidden: true, readonly: true, "\n{skill.markdown}" }
                                    button {
                                        r#type: "button",
                                        "data-copy-source": "copy-source",
                                        class: "group hover:text-accent dark:hover:text-accent flex cursor-pointer items-center gap-2 bg-white/90 px-3 py-2 text-sm font-medium text-zinc-800 shadow-lg ring-1 shadow-zinc-800/5 ring-zinc-900/5 backdrop-blur-sm transition dark:bg-zinc-800/90 dark:text-zinc-200 dark:ring-white/10 dark:hover:ring-white/20",
                                        Icon {
                                            view_box: "0 0 16 16",
                                            class: "h-4 w-4 stroke-current",
                                            fill: "none".to_string(),
                                            inner: r#"<path d="M5.75 5.75h5.5a1 1 0 0 1 1 1v5.5a1 1 0 0 1-1 1h-5.5a1 1 0 0 1-1-1v-5.5a1 1 0 0 1 1-1Z" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/><path d="M3.75 9.75h-1a1 1 0 0 1-1-1v-5.5a1 1 0 0 1 1-1h5.5a1 1 0 0 1 1 1v1" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>"#,
                                        }
                                        span { "data-copy-label": "", "Copy as MD" }
                                    }
                                }
                            }
                            p { class: "order-first flex items-center text-base text-zinc-400 dark:text-zinc-500",
                                "{skill.description}"
                            }
                        }
                        div { class: "prose dark:prose-invert mt-8", dangerous_inner_html: skill.html }
                    }
                }
            }
        }
    }
}

// ───────────────────────────── cards ─────────────────────────────

#[component]
fn CardGrid(cards: Vec<Card>, contain: bool) -> Element {
    let img_class = if contain {
        "h-8 w-8 object-contain"
    } else {
        "h-8 w-8"
    };

    rsx! {
        div { class: "mt-16 sm:mt-20",
            div { class: "md:pl-6 md:dark:border-zinc-700/40",
                div { class: "flex flex-col space-y-16",
                    ul { role: "list", class: "grid grid-cols-1 gap-x-12 gap-y-16 sm:grid-cols-2 lg:grid-cols-3",
                        for card in cards {
                            li { key: "{card.name}", class: "group relative flex flex-col items-start",
                                div {
                                    class: "relative z-10 flex h-12 w-12 items-center justify-center bg-white shadow-md ring-1 shadow-zinc-800/5 ring-zinc-900/5 dark:border dark:border-zinc-700/50 dark:bg-zinc-800 dark:ring-0",
                                    style: "background-color:{card.bg_color}",
                                    img {
                                        alt: "",
                                        loading: "lazy",
                                        decoding: "async",
                                        width: if contain { None } else { Some("32") },
                                        height: if contain { None } else { Some("32") },
                                        class: img_class,
                                        src: "{card.image}",
                                    }
                                }
                                h2 { class: "mt-6 text-base font-semibold text-zinc-800 dark:text-zinc-100",
                                    div { class: HOVER_BG }
                                    a { href: "{card.url}", target: "_blank", rel: "noopener noreferrer",
                                        span { class: "absolute -inset-x-4 -inset-y-6 z-20 sm:-inset-x-6" }
                                        span { class: "relative z-10", "{card.name}" }
                                    }
                                }
                                p { class: "relative z-10 mt-2 text-sm text-zinc-600 dark:text-zinc-400", "{card.description}" }
                                p { class: "relative z-10 mt-6 flex text-sm font-medium text-zinc-400 transition group-hover:text-teal-500 dark:text-zinc-200",
                                    Icon {
                                        view_box: "0 0 24 24",
                                        class: "h-6 w-6 flex-none",
                                        inner: LINK_ICON,
                                    }
                                    span { class: "ml-2", "{card.hostname()}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

const LINK_ICON: &str = r#"<path d="M15.712 11.823a.75.75 0 1 0 1.06 1.06l-1.06-1.06Zm-4.95 1.768a.75.75 0 0 0 1.06-1.06l-1.06 1.06Zm-2.475-1.414a.75.75 0 1 0-1.06-1.06l1.06 1.06Zm4.95-1.768a.75.75 0 1 0-1.06 1.06l1.06-1.06Zm3.359.53-.884.884 1.06 1.06.885-.883-1.061-1.06Zm-4.95-2.12 1.414-1.415L12 6.344l-1.415 1.413 1.061 1.061Zm0 3.535a2.5 2.5 0 0 1 0-3.536l-1.06-1.06a4 4 0 0 0 0 5.656l1.06-1.06Zm4.95-4.95a2.5 2.5 0 0 1 0 3.535L17.656 12a4 4 0 0 0 0-5.657l-1.06 1.06Zm1.06-1.06a4 4 0 0 0-5.656 0l1.06 1.06a2.5 2.5 0 0 1 3.536 0l1.06-1.06Zm-7.07 7.07.176.177 1.06-1.06-.176-.177-1.06 1.06Zm-3.183-.353.884-.884-1.06-1.06-.884.883 1.06 1.06Zm4.95 2.121-1.414 1.414 1.06 1.06 1.415-1.413-1.06-1.061Zm0-3.536a2.5 2.5 0 0 1 0 3.536l1.06 1.06a4 4 0 0 0 0-5.656l-1.06 1.06Zm-4.95 4.95a2.5 2.5 0 0 1 0-3.535L6.344 12a4 4 0 0 0 0 5.656l1.06-1.06Zm-1.06 1.06a4 4 0 0 0 5.657 0l-1.061-1.06a2.5 2.5 0 0 1-3.535 0l-1.061 1.06Zm7.07-7.07-.176-.177-1.06 1.06.176.178 1.06-1.061Z" fill="currentColor"/>"#;

#[component]
fn ProjectsPage() -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Projects & Contributions", description: "Software I've built or contributed to." }
            CardGrid { cards: data::projects(), contain: false }
        }
    }
}

#[component]
fn UsesPage() -> Element {
    rsx! {
        PageFrame {
            Hero { title: "Uses", description: "Hardware & Software that I use to get things done." }
            CardGrid { cards: data::toolbox(), contain: true }
        }
    }
}

#[component]
fn NotFoundPage() -> Element {
    rsx! {
        PageFrame {
            div { class: "max-w-2xl",
                h1 { class: "text-4xl font-bold tracking-tight text-zinc-800 sm:text-5xl dark:text-zinc-100", "Not found" }
                p { class: "mt-6 text-base text-zinc-600 dark:text-zinc-400",
                    "The page you are looking for does not exist."
                }
            }
        }
    }
}
