---
title: "Installing The Rust Programming Language on Windows"
description: "A tutorial on installing the Rust Programming Language on Windows."
categories: [rust, tutorial, windows, install]
icon: rust
date: 2021-09-13
preview_image_url: "https://images.unsplash.com/photo-1609131926660-3bec9d472529?ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&ixlib=rb-1.2.1&auto=format&fit=crop&w=2070&q=80"
published: true
---

## Motivation

In the past days I've been using Unix based systems to do my software
development work, macOS and Ubuntu are both my main operative systems nowadays.

But Windows is getting closer as well, as I get more involved into systems
programming, I'm also getting into writing Rust crates which must be supported in
different platforms, such as macOS, Linux and Windows.

Currently I'm working on a crate called [local-ip-address](https://github.com/LeoBorai/local-ip-address).

The main goal of this crate is to list system's network interfaces along
with related data such as interface name, interface family (AFINET or AFINET6 for instance),
IP address, subnet mask and any other relevant properties.

Given that every system has a particular way to gather network interfaces
details, I decided to install Windows in my PC as a dual-boot option along with Ubuntu.

This will give me first-class access to the popular Win32 API, which I'm using through [windows-rs](https://github.com/microsoft/windows-rs) crate.

After having Windows up and running, I'm also installing Rust on Windows and I'm documenting
it for future references.

## Installing Rust on Windows

First you will have to install the Microsoft C++ build tools for Visual Studio 2013
or a newer version. This is a dependency to have Rust installed on Windows with full
compatibility.

Is important to mention that advanced users taergeting the GNU ABI or doing a different
setup may not need to have the C++ Build Tools installed in their systems as pointed out
by the Rustup binary:

![Screenshot from https://www.rust-lang.org/tools/install](/notes/images/001-install-rust-binary.png)

In order to have C++ Build Tools installed, visit the [Visual C++ Build Tools website](https://visualstudio.microsoft.com/visual-cpp-build-tools/) to download and install it.

Once Visual Studio Installer is ready, choose the "Desktop development with C++" workload.

![Visual Studio Installer with the C++ Workload Selected](/notes/images/001-vs-installer.png)

Visual Studio Build Tools will be downloaded and installed in your system, this may take
some minutes.

![Visual Studio Installer with the C++ Workload Selected](/notes/images/001-vs-build-tools-installation.png)

When Visual Studio installer have finished installing the C++ Build Tools, you will be
ready to install Rust!

Go to the official [Rust Install](https://www.rust-lang.org/tools/install) website, and
download the executable for your architecture.

![Rust Install Website](/notes/images/001-install-rust-website.png)

Execute the binary, you must see the following output:

```powershell
The Cargo home directory located at:

  C:\Users\Esteban\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

```powershell
  C:\Users\Esteban\.cargo\bin
```

This path will then be added to your PATH environment variable by
modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

```
Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>
```

Press `1` key and then the `Enter` key.

This will download: `cargo`, `clippy` and `rust-docs`, the last two
are components for the `cargo` CLI, if you are familiar with Rust you may
heard of them before.

If you are not, `clippy` is a linter, which gives you hints into best practices
and suggestions for your Rust code, making it more efficient, conscice and thus
clean.

`rust-docs` enables the `cargo doc` command, which extracts documentation from the
source files of the crates avaialble in your project and builds an HTML you can use
offline in your system.

This HTML is the same you will find if you visit [docs.rs](https://docs.rs).

If you got here, you should be good to go! Please, open an issue or pull request if you
find any issue with this document, I will be happy to follow up.
