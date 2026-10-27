---
title: "Setup Google Cloud SQL Proxy on macOS"
description: "Setting up Google Cloud  with SQL Proxy on macOS using Homebrew"
categories: [homebrew, macos, gcloud, sql, proxy, setup, sdk]
icon: gcp
date: 2022-02-07
preview_image_url: "https://images.unsplash.com/photo-1597008641621-cefdcf718025?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1039&q=80"
published: true
---

From almost two years now I have been working with Google Cloud Services on a
daily basis. Debugging cloud run instances, app engine instances and checking
on SQL database instances performance and open connections are some of the
tasks I have been doing on Google Cloud Services.

One of the tools I use the most is the Cloud SQL Proxy, which lets me reach
database instances running on GCloud through running a single command in my
terminal. Cloud SQL Proxy basically opens a TCP connection in my machine which
I can bind to and communicate to my database instance just like I'm running it
locally its a very good experience in my opinion.

> Before going any further it's important that you have Xcode command-line tools installed. You can run `xcode-select --install` from your terminal to have it installed.

I'm going to install Google Cloud CLI using [Homebrew](https://brew.sh). If you
don't know what Homebrew is then you must understand that Homebrew is a package
manager for macOS, similar ⏤ but not the same ⏤ to the [apt-get](https://linux.die.net/man/8/apt-get)
command if you have used Ubuntu before or [Chocolatey](https://chocolatey.org) if
you have used it on Windows.

You can install Homebrew by opening your terminal and executing the following
command:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```
[Source](https://brew.sh)

## Installing Google Cloud SDK

With `brew` ⏤ the command used to execute Homebrew tasks ⏤ you can install
the Google Cloud SDK binary by executing:

```bash
brew install cask google-cloud-sdk
```

The installation may take around 5 minutes to complete, you will see a lot of
output from Homebrew while performing its tasks. When the installation is
complete you will see output similar to this:

```
Welcome to the Google Cloud CLI!
Beginning update. This process may take several minutes.


Your current Cloud SDK version is: 371.0.0
Installing components from version: 371.0.0

┌─────────────────────────────────────────────────────────────────────────────┐
│                     These components will be installed.                     │
├─────────────────────────────────────────────────────┬────────────┬──────────┤
│                         Name                        │  Version   │   Size   │
├─────────────────────────────────────────────────────┼────────────┼──────────┤
│ BigQuery Command Line Tool                          │     2.0.73 │  1.0 MiB │
│ BigQuery Command Line Tool (Platform Specific)      │     2.0.73 │  < 1 MiB │
│ Cloud Storage Command Line Tool                     │        5.6 │  8.1 MiB │
│ Cloud Storage Command Line Tool (Platform Specific) │        5.6 │  < 1 MiB │
│ GoogleCloud CLI Core Libraries (Platform Specific)  │ 2022.01.28 │  < 1 MiB │
│ anthoscli                                           │     0.2.22 │ 46.5 MiB │
│ gcloud cli dependencies                             │ 2021.04.16 │  < 1 MiB │
└─────────────────────────────────────────────────────┴────────────┴──────────┘

For the latest full release notes, please visit:
  https://cloud.google.com/sdk/release_notes
```

This means that Google Cloud CLI is installed in your macOS computer with no
issues!.

You will also notice that some other commands for Google Cloud SDK are installed
by default:

```
╔════════════════════════════════════════════════════════════╗
╠═ Creating update staging area                             ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: BigQuery Command Line Tool                   ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: BigQuery Command Line Tool (Platform Spec... ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: Cloud Storage Command Line Tool              ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: Cloud Storage Command Line Tool (Platform... ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: Default set of gcloud commands               ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: GoogleCloud CLI Core Libraries (Platform ... ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: anthoscli                                    ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: anthoscli                                    ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Installing: gcloud cli dependencies                      ═╣
╚════════════════════════════════════════════════════════════╝
╔════════════════════════════════════════════════════════════╗
╠═ Creating backup and activating new installation          ═╣
╚════════════════════════════════════════════════════════════╝
```

Homebrew will ask you to install shell completions, you can do it by running the
commands printed on the terminal:

```
To install shell completions, add this to your profile:

  for bash users
    source "/usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/completion.bash.inc"

  for zsh users
    source "/usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/completion.zsh.inc"
```

## Using the `gcloud` command

After installing `google-cloud-sdk` the `gcloud` command will be available.
Make sure it is installed by execting `gcloud --version` in your terminal:

```bash
gcloud --version
```

Output:

```
Google Cloud SDK 371.0.0
bq 2.0.73
core 2022.01.28
gsutil 5.6
```

Let's check which components are available, you can run:

```bash
gcloud components list
```

Output:

```
Your current Cloud SDK version is: 371.0.0
The latest available version is: 371.0.0

┌────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                 Components                                                 │
├───────────────┬──────────────────────────────────────────────────────┬──────────────────────────┬──────────┤
│     Status    │                         Name                         │            ID            │   Size   │
├───────────────┼──────────────────────────────────────────────────────┼──────────────────────────┼──────────┤
│ Not Installed │ App Engine Go Extensions                             │ app-engine-go            │  4.1 MiB │
│ Not Installed │ Appctl                                               │ appctl                   │ 18.5 MiB │
│ Not Installed │ Cloud Bigtable Command Line Tool                     │ cbt                      │  7.9 MiB │
│ Not Installed │ Cloud Bigtable Emulator                              │ bigtable                 │  5.9 MiB │
│ Not Installed │ Cloud Datalab Command Line Tool                      │ datalab                  │  < 1 MiB │
│ Not Installed │ Cloud Datastore Emulator                             │ cloud-datastore-emulator │ 18.4 MiB │
│ Not Installed │ Cloud Firestore Emulator                             │ cloud-firestore-emulator │ 40.5 MiB │
│ Not Installed │ Cloud Pub/Sub Emulator                               │ pubsub-emulator          │ 60.7 MiB │
│ Not Installed │ Cloud SQL Proxy                                      │ cloud_sql_proxy          │  7.6 MiB │
│ Not Installed │ Google Cloud Build Local Builder                     │ cloud-build-local        │  6.2 MiB │
│ Not Installed │ Google Container Registry's Docker credential helper │ docker-credential-gcr    │  2.2 MiB │
│ Not Installed │ Kustomize                                            │ kustomize                │  7.6 MiB │
│ Not Installed │ Minikube                                             │ minikube                 │ 26.8 MiB │
│ Not Installed │ Nomos CLI                                            │ nomos                    │ 23.6 MiB │
│ Not Installed │ On-Demand Scanning API extraction helper             │ local-extract            │ 13.0 MiB │
│ Not Installed │ Skaffold                                             │ skaffold                 │ 19.2 MiB │
│ Not Installed │ anthos-auth                                          │ anthos-auth              │ 18.0 MiB │
│ Not Installed │ config-connector                                     │ config-connector         │ 49.7 MiB │
│ Not Installed │ gcloud Alpha Commands                                │ alpha                    │  < 1 MiB │
│ Not Installed │ gcloud Beta Commands                                 │ beta                     │  < 1 MiB │
│ Not Installed │ gcloud app Java Extensions                           │ app-engine-java          │ 51.6 MiB │
│ Not Installed │ gcloud app PHP Extensions                            │ app-engine-php           │ 21.9 MiB │
│ Not Installed │ gcloud app Python Extensions                         │ app-engine-python        │  7.8 MiB │
│ Not Installed │ gcloud app Python Extensions (Extra Libraries)       │ app-engine-python-extras │ 26.4 MiB │
│ Not Installed │ kpt                                                  │ kpt                      │ 12.2 MiB │
│ Not Installed │ kubectl                                              │ kubectl                  │  < 1 MiB │
│ Not Installed │ kubectl-oidc                                         │ kubectl-oidc             │ 18.0 MiB │
│ Not Installed │ pkg                                                  │ pkg                      │          │
│ Installed     │ BigQuery Command Line Tool                           │ bq                       │  1.0 MiB │
│ Installed     │ Cloud Storage Command Line Tool                      │ gsutil                   │  8.1 MiB │
│ Installed     │ GoogleCloud CLI Core Libraries                       │ core                     │ 21.9 MiB │
└───────────────┴──────────────────────────────────────────────────────┴──────────────────────────┴──────────┘
To install or remove components at your current SDK version [371.0.0], run:
  $ gcloud components install COMPONENT_ID
  $ gcloud components remove COMPONENT_ID

To update your SDK installation to the latest version [371.0.0], run:
  $ gcloud components update
```

You can see `cloud_sql_proxy` is on the list of components and it's also not
installed already. Let's install it by executing:

```bash
gcloud components install cloud_sql_proxy
```

Output:

```
Your current Cloud SDK version is: 371.0.0
Installing components from version: 371.0.0

┌─────────────────────────────────────────────────────────────────────────┐
│                   These components will be installed.                   │
├─────────────────────────────┬─────────────────────┬─────────────────────┤
│             Name            │       Version       │         Size        │
├─────────────────────────────┼─────────────────────┼─────────────────────┤
│ Cloud SQL Proxy             │              1.27.0 │             7.6 MiB │
└─────────────────────────────┴─────────────────────┴─────────────────────┘

For the latest full release notes, please visit:
  https://cloud.google.com/sdk/release_notes

Do you want to continue (Y/n)?
```

Press `Y` and then the `return` key. The installation will begin and you will
have to wait a couple minutes until it's ready.

## Adding `cloud_sql_proxy` to the path

When installation is completed you can attempt to execute `cloud_sql_proxy`,
but you will notice that such binary is not available.

```bash
cloud_sql_proxy
```

Output:

```
zsh: command not found: cloud_sql_proxy
```

This is because `gcloud` uses a directory which is not being taken into account
in the `PATH` environment variable. You must add the directory where gcloud is
storing the binaries to your `PATH` environment variable in order to make it
reachable when executing the `cloud_sql_proxy` command.

Let's use `which` command to check where is `gcloud` binary stored:

```bash
which gcloud
```

Output:

```
/usr/local/bin/gcloud
```

If you go to `/usr/local/bin/` you will notice that every file under `/bin/` is
a symbolic link to a binary which is actually stored in its own installation
directory.

Change directories to `/usr/local/bin/` by executing:

```bash
cd /usr/local/bin
```

Then list entries by executing the `l` command and find the symbolic link to
the `gcloud` binary. The entry must look similar to:

```
gcloud -> /usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/bin/gcloud
```

You can see that `gcloud` binary is actually stored on:

```
/usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/bin
```

So lets change directories to that path:

```bash
cd /usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/bin
```

If you run `ls` command on `/usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/bin`
you will see that `cloud_sql_proxy` is listed there. This means that you have to
make sure that this directory is listed in our `PATH` environment variable so
it's reachable when executing: `cloud_sql_proxy`.

Open you `rc` file to add the path to the `PATH` environment variable. If you
are using BASH then you are interested on `.bashrc`. I'm a Oh My Zsh user so
I'm interested on `.zshrc` instead.

```bash
vim ~/.zshrc
```

Add the following line to the very bottom of the file and save it.

```bash
PATH="$PATH:/usr/local/Caskroom/google-cloud-sdk/latest/google-cloud-sdk/bin"
```

Finally use `source` command to load the terminal instance configuration:

```bash
# Bash
source ~/.bashrc

# ZSH/Oh My Zsh
source ~/.zshrc
```

Make sure `cloud_sql_proxy` is reachable:

```bash
cloud_sql_proxy --version
```

Output:

```
Cloud SQL Auth proxy: 1.27.0+darwin.amd64
```
