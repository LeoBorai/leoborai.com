---
title: "Rust GitHub Actions Release Workflow"
description: "Setting up a GitHub Actions Workflow to tag and release a Rust Crate via Workflow Dispatch"
categories: [rust, github, cd, ci, automation]
icon: rust
date: 2023-11-04
preview_image_url: "https://unsplash.com/photos/8gr6bObQLOI/download?ixid=M3wxMjA3fDB8MXxzZWFyY2h8NjB8fGF1dG9tYXRpb258ZW58MHx8fHwxNjk5MTI3MTI3fDA&force=true&w=640"
published: true
---

## Motivation

Maintianing many open source packages is time consuming, you have pull requests
to review, security issues to concern about and also patch, open tracking issues
to continuosly improve and also keep an eye on updates and improvements.

This makes any automation improvements a big deal, whenever you can automate
these tasks will allow you to have more time to focus in actual work.

Dependency management can be addressed with Dependabot which is pretty easy to
setup, just setup a `.github/dependabot.yml` with the following contents and
you will get weekly updates on dependencies:

```yml
version: 2
updates:
  - package-ecosystem: 'cargo'
    directory: '/'
    schedule:
      interval: 'weekly'
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: 'weekly'
```

This will open a bunch on PRs every monday, so its recommeded to setup a CI
workflow to at least check tests and build integrity.

By having these CI workflows settled, you can then provide a couple workflows
for approving and merging PRs.

The following workflow will approve PRs from dependabot when CI passes:

```yml
name: Dependabot auto-approve
on: pull_request_target

permissions:
  pull-requests: write

jobs:
  dependabot:
    runs-on: ubuntu-latest
    if: ${{ github.actor == 'dependabot[bot]' }}
    steps:
      - name: Dependabot metadata
        id: metadata
        uses: dependabot/fetch-metadata@v1.1.1
        with:
          github-token: "${{ secrets.GITHUB_TOKEN }}"
      - name: Approve a PR
        run: gh pr review --approve "$PR_URL"
        env:
          PR_URL: ${{github.event.pull_request.html_url}}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

And this other will merge approved PRs:

```yml
name: Dependabot auto-merge
on: pull_request_target

permissions:
  pull-requests: write
  contents: write

jobs:
  dependabot:
    runs-on: ubuntu-latest
    if: ${{ github.actor == 'dependabot[bot]' }}
    steps:
      - name: Dependabot metadata
        id: metadata
        uses: dependabot/fetch-metadata@v1.1.1
        with:
          github-token: "${{ secrets.GITHUB_TOKEN }}"
      - name: Enable auto-merge for Dependabot PRs
        if: ${{steps.metadata.outputs.update-type == 'version-update:semver-patch'}}
        run: gh pr merge --auto --squash "$PR_URL"
        env:
          PR_URL: ${{github.event.pull_request.html_url}}
          GITHUB_TOKEN: ${{secrets.GITHUB_TOKEN}}
```

> Kudos to GitHub 🍻 for providing these workflows! The [official guide is here][1]

But after this setup I still had the need for releasing patch releases by hand,
my task list was:

1. Pull latest contents from GitHub
2. Update tag in `Cargo.toml`
3. Create tag using `git tag`
4. Push to `main` using such tag
5. GitHub Workflow ran to publish to crates.io

But that workflow had many issues, repetition af manual tasks are error prone,
sometimes I had to cancel publish work because a mistaken version number was
bumped, or because I forgot to update `Cargo.toml`.

So I had to provide one more workflow, one that automates my task list.

## Preparing a Workflow to Bump Version and Push

I was inspired by NPM's `version` command, it basically covered my needs, when
you run `npm version <major | minor | patch>`, you have the `version` field
from `package.json` updated with a `+1` on the desired version digit, you also
get `git tag` run with the new version, thats great!

With that in mind, and as an acceptance criteria for my workflow, I started to
investigate on GitHub, looking for a `cargo` plugin that covers this for me.

And thats how I found `cargo-set-version` which was part of [`cargo-edit`][2] crate.

So the set of tasks to perform before publishing are:

1. Check crate integrity with a _dry run_ on `cargo publish`
2. Setup `cargo-set-version` as part of the CI Workflow (This involves setting up Rust)
3. Update `Cargo.toml` version
4. Commit changes to upstream pushing with tags included
5. Login to `Crates.io`
6. Upload the crate
7. Create GitHub Release

Let me walk you through the proccess.

1. Create a `Release` workflow on `.github/workflow/release.yml`

2. Provide workflow dispatch support with an input for version number to bump

```yml
name: Release

on:
  workflow_dispatch:
    inputs:
      version:
        type: choice
        required: true
        description: 'Version number to bump'
        options:
          - patch
          - minor
          - major
```

3. As we need to commit and push, permissions to write must be granted:

```yml
permissions:
  contents: write
  issues: write
  pull-requests: write
```

4. Now lets implement jobs, the first job (`publish-dry-run`) will check for
crate integrity, the second (`release`) will perform release work:

```yml
jobs:
  publish-dry-run:
    name: "Runs cargo publish --dry-run"
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Source Code
        uses: actions/checkout@v1

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable

      - name: Check crate publish
        run: cargo publish --dry-run
```

Then `release` job would look like this:

```yml
  release:
    name: Create Release
    needs: publish-dry-run
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v2

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true

      - name: Install `cargo-edit`
        run: cargo install cargo-edit

      - id: cargo-set-version
        name: Set Version
        run: cargo set-version --bump ${{ inputs.version }}

      - name: Set Crate Version as Environment Variable
        run: |
          CARGO_TOML_VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
          echo "CRATE_VERSION=$CARGO_TOML_VERSION" >> $GITHUB_ENV

      - name: Create Commit
        run: |
          git config --global user.name 'github-actions[bot]'
          git config --global user.email 'github-actions[bot]@users.noreply.github.com'
          git add .
          git commit -m "chore: bump version to v$CRATE_VERSION"
          git push origin main --follow-tags

      - name: Login to Crates.io
        run: cargo login ${CRATES_IO_TOKEN}
        env:
          CRATES_IO_TOKEN: ${{ secrets.CRATES_IO_TOKEN }}

      - name: Publish crate
        run: cargo publish

      - name: Create Release
        id: create_release
        uses: actions/github-script@v5
        with:
          script: |
            await github.request(`POST /repos/${{ github.repository }}/releases`, {
              tag_name: "v${{ env.CRATE_VERSION }}",
              generate_release_notes: true
            });
```

I have been using this workflow since July 2023 and it works pretty nice!
You can [see how it performs here][3].

[Here][4] is the full source if you get lost!

## Conclusion

Automating repetitive tasks will always allow you to focus on more interesting
ones and at the same time will reduce errors dramatically. Releasing new
versions should be easy to achieve, even if you are AFK you should be able to
merge PRs and release versions of your packages, you never know who may need
a patch on one of your crates urgently!

[1]: https://docs.github.com/en/code-security/dependabot/working-with-dependabot/automating-dependabot-with-github-actions
[2]: https://crates.io/crates/cargo-edit
[3]: https://github.com/whizzes/pxid/actions/workflows/release.yml
[4]: https://github.com/whizzes/pxid/blob/e5125c8eaed7492a5c7143e9e61e9ab818d31a00/.github/workflows/release.yml#L1
