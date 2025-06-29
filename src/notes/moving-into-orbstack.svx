---
title: "Moving into OrbStack after a year using Rancher Desktop"
description: "A primer on Git repositories, commits & branches"
categories: [kubernetes, docker, rancher, orbstack, client, macos]
icon: docker
date: 2023-08-29
preview_image_url: "https://images.unsplash.com/photo-1598056036946-3cfa8493bfdf?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=1740&q=1"
published: true
---

Almost a year ago I started using Rancher Desktop as my Docker and Kubernetes
client. I enjoy the fact that I didn't need to know a lot about Kubernetes to
easily jump into spinning up clusters, configure port fowarding and manage pods.

## Uninstalling Rancher Desktop on macOS

Clean up any remaining Docker Containers and Volumes using the `system prune`
subcommand.

```bash
docker system prune --all --volumes --force
```

```bash
# -- snip --
pog7t3hmzcyurdvypx2fgoq7u
w8t2eav93yh12w0p9qaghdg5u
uh5vhr6tjhv73boiqo8d7bvur
nsm2ohhywv95uqk9bx2hcgpwb
yyt452o7gjp9xxmwu7djd1efc
n3ni6ght2y7pd011modusutep

Total reclaimed space: 21.376GB
```

> This command cleaned up around 20 GBs of disk from my system.

Then, make sure Rancher Desktop is not running anymore, you can do this by
clicking on the Rancher Desktop icon on the status bar and then clicking on
the `Quit Rancher Desktop` option.

After making sure we dont have more Docker Containers running, we are good to
go and its time to delete the application from the `~/Applications` directory.

![Move Application to Trash](/images/notes/010-applications.png)

After deleting the Application Package, you will also want to delete remaining
files, most macOS applications store files in other system directories, such as
caches, logs, preferences and other user data.

These files lives on `~/Library` directory, search for relevant files with a
path starting with `rancher-desktop`.

If you attempt to find any file containing `rancher-desktop` like this:

```bash
find ~/Library -name "rancher-desktop"
```

You will encounter lots of `Operation not permitted` messages:

```bash
find: ~/Library/Application Support/CallHistoryTransactions: Operation not permitted
find: ~/Library/Application Support/com.apple.sharedfilelist: Operation not permitted
find: ~/Library/Application Support/Knowledge: Operation not permitted
```

Instead you can use `open ~/Library` and navigate from there using the macOS
Finder.

![Application Support](/images/notes/010-app-support.png)

Also check for the `Caches` directory under `~/Library/Application Support/Caches`

![Caches](/images/notes/010-app-support-caches.png)

Another directory you should clean is `~/Library/Preferences`, there you will
find a directory and a `plist` file.

![Preferences](/images/notes/010-preferences.png)

Logs is another place to search as well.

![Logs](/images/notes/010-logs.png)

## Enter OrbStack

You can download OrbStack from the official website visiting [docs.orbstack.dev/quick-start][1],
or using Homebrew!

```bash
brew install orbstack
```

And here is when the fun begins, OrbStack will allow you to spin up Linux
Machines or Docker. But the great thing is that you can do both and you won't
need a Linux machine to run containers!

![OrbStack Linux](/images/notes/010-orbstack-linux.png)

> Back in Rancher Desktop, I had a [QEMU][2] instance running, eating 40%-60% of
> my CPU, sometimes just to spin up a PostgreSQL instance.

You can list running containers and manage Linux VMs just by clicking a couple
buttons, its super fast and easy to use, that without mentioning the native macOS GUI.

![OrbStack Home](/images/notes/010-orbstack-home.png)

[1]: https://docs.orbstack.dev/quick-start
[2]: https://www.qemu.org
