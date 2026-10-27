//! Static site data. Long SVG/URL lists live in `assets/*.json`.

use serde::Deserialize;

pub const SITE_NAME: &str = "Leo Borai";
pub const SITE_ROLE: &str = "Software Developer";

pub const NAV_LINKS: [(&str, &str); 5] = [
    ("Home", "/"),
    ("Notes", "/notes"),
    ("Skills", "/skills"),
    ("Projects", "/projects"),
    ("Uses", "/uses"),
];

#[derive(Clone, PartialEq, Deserialize)]
pub struct SocialLink {
    pub label: String,
    pub href: String,
    #[serde(rename = "viewBox")]
    pub view_box: String,
    pub path: String,
    #[serde(default)]
    pub layers: Vec<String>,
    #[serde(default)]
    pub evenodd: bool,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Card {
    pub name: String,
    pub url: String,
    pub description: String,
    pub image: String,
    #[serde(rename = "bgColor")]
    pub bg_color: String,
}

impl Card {
    pub fn hostname(&self) -> &str {
        self.url
            .split("://")
            .nth(1)
            .unwrap_or(&self.url)
            .split(['/', '?', '#'])
            .next()
            .unwrap_or("")
    }
}

pub fn social_links() -> Vec<SocialLink> {
    serde_json::from_str(include_str!("../assets/social.json")).expect("assets/social.json")
}

pub fn projects() -> Vec<Card> {
    serde_json::from_str(include_str!("../assets/projects.json")).expect("assets/projects.json")
}

pub fn toolbox() -> Vec<Card> {
    serde_json::from_str(include_str!("../assets/uses.json")).expect("assets/uses.json")
}

#[derive(PartialEq)]
pub struct Skill {
    pub name: &'static str,
    pub logo: Option<&'static str>,
    pub href: Option<&'static str>,
}

const fn skill(name: &'static str, logo: &'static str, href: &'static str) -> Skill {
    Skill {
        name,
        logo: Some(logo),
        href: if href.is_empty() { None } else { Some(href) },
    }
}

pub const SKILL_GROUPS: &[(&str, &[Skill])] = &[
    (
        "Languages",
        &[
            skill(
                "Rust",
                "/img/techstack/rust.svg",
                "https://www.rust-lang.org",
            ),
            skill(
                "TypeScript",
                "/img/techstack/typescript.svg",
                "https://www.typescriptlang.org",
            ),
            skill(
                "Python",
                "/img/techstack/python.svg",
                "https://www.python.org",
            ),
            skill("SQL", "/img/techstack/postgresql.svg", ""),
        ],
    ),
    (
        "Frameworks",
        &[
            skill(
                "SvelteKit",
                "/img/techstack/svelte.svg",
                "https://svelte.dev",
            ),
            skill(
                "Next.js",
                "/img/techstack/nextdotjs.svg",
                "https://nextjs.org",
            ),
            skill(
                "Ember.js",
                "/img/techstack/emberdotjs.svg",
                "https://emberjs.com",
            ),
        ],
    ),
    (
        "Infrastructure",
        &[
            skill(
                "Docker",
                "/img/techstack/docker.svg",
                "https://www.docker.com",
            ),
            skill(
                "Cloudflare",
                "/img/techstack/cloudflare.svg",
                "https://www.cloudflare.com",
            ),
            skill("Fly.io", "/img/techstack/flydotio.svg", "https://fly.io"),
            skill(
                "GCP",
                "/img/techstack/googlecloud.svg",
                "https://cloud.google.com",
            ),
        ],
    ),
    (
        "Workflow",
        &[
            skill("Git", "/img/techstack/git.svg", "https://git-scm.com"),
            skill(
                "Claude Code",
                "/img/techstack/claude.svg",
                "https://www.anthropic.com/claude-code",
            ),
        ],
    ),
];
