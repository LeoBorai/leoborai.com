//! Compiles the Tailwind CSS bundle so no Node/npm/dx tooling is needed.
//!
//! Downloads the pinned, checksum-verified Tailwind standalone binary (it
//! bundles the `forms` and `typography` plugins), runs it over `assets/app.css`
//! and writes the minified result to `$OUT_DIR/app.css`, which `main.rs`
//! embeds with `include_str!`.
//!
//! Set `TAILWINDCSS_BIN` to use a local binary instead of downloading.

use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

const VERSION: &str = "4.3.3";

/// (target os, target arch) -> (release asset, sha256)
fn asset() -> (&'static str, &'static str) {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    match (os.as_str(), arch.as_str()) {
        ("linux", "x86_64") => (
            "tailwindcss-linux-x64",
            "dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a",
        ),
        ("linux", "aarch64") => (
            "tailwindcss-linux-arm64",
            "55fd0b241214eff3de1e8ee4f22796662f2d2e7a49bcfca7477cfd0bac398195",
        ),
        ("macos", "aarch64") => (
            "tailwindcss-macos-arm64",
            "cdf646702987a743464dff4d9c60fd4480d1c1e73dd819a9a67f1078815dce9d",
        ),
        ("macos", "x86_64") => (
            "tailwindcss-macos-x64",
            "7922e0953f2110c05976e3bf58f14e643d90427575e766b7d433f5f80cbee7e1",
        ),
        ("windows", "x86_64") => (
            "tailwindcss-windows-x64.exe",
            "e0e260ce048014e9268f6237ff18f8ccf02cef521cbd0ae04e82c2cdf7aa3955",
        ),
        other => panic!("no Tailwind standalone binary for {other:?}; set TAILWINDCSS_BIN"),
    }
}

fn download(out_dir: &Path) -> PathBuf {
    let (name, sha) = asset();
    let bin = out_dir.join(format!("tw-{VERSION}-{name}"));

    if bin.exists() {
        return bin;
    }

    let url =
        format!("https://github.com/tailwindlabs/tailwindcss/releases/download/v{VERSION}/{name}");
    println!("cargo:warning=downloading {url}");

    let mut body = Vec::new();
    ureq::get(&url)
        .call()
        .unwrap_or_else(|e| panic!("failed to download {url}: {e}"))
        .into_body()
        .into_reader()
        .read_to_end(&mut body)
        .expect("failed to read Tailwind binary");

    let digest = format!("{:x}", Sha256::digest(&body));
    assert_eq!(digest, sha, "checksum mismatch for {name}");

    let tmp = bin.with_extension("part");
    fs::write(&tmp, &body).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&tmp, fs::Permissions::from_mode(0o755)).unwrap();
    }

    fs::rename(&tmp, &bin).unwrap();
    bin
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-env-changed=TAILWINDCSS_BIN");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bin = match env::var_os("TAILWINDCSS_BIN") {
        Some(path) => PathBuf::from(path),
        None => download(&out_dir),
    };

    let status = Command::new(&bin)
        .current_dir(env::var("CARGO_MANIFEST_DIR").unwrap())
        .args(["-i", "assets/app.css", "-o"])
        .arg(out_dir.join("app.css"))
        .arg("--minify")
        .status()
        .unwrap_or_else(|e| panic!("failed to run {}: {e}", bin.display()));

    assert!(status.success(), "tailwindcss failed");
}
