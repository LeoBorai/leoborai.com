---
title: "Use Zed as IDE for Java on macOS"
description: "How I setup Java environment on macOS to use Zed as my main IDE."
categories: [java, zed, macos]
icon: rust
date: 2025-09-27
preview_image_url: "https://images.unsplash.com/photo-1506322526487-d8a94af63688?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&w=640"
published: true
---

I have been using Zed for a long time now, I love how performant and extensible it is
as a IDE. Most of the code I write nowadays is Rust, then I also write TypeScript and
Svelte.

However I happened to need to write some Java recently, before around 2016 I used NetBeans
as my primary IDE for Java. But I'm so used to Zed that I want to continue using it for Java.

## Install the OpenJDK

The OpenJDK is the open source implementation of the Java Runtime. It allows you to build
and run Java applications. On macOS, the easiest way to install it is via Homebrew.

```bash
brew install openjdk
```

Then update your `...rc` file to include the OpenJDK binaries in your `PATH`.

```
export PATH="/opt/homebrew/opt/openjdk/bin:$PATH"
export CPPFLAGS="-I/opt/homebrew/opt/openjdk/include"
```

> Remember to save the file and also run `source ~/.zshrc` or `source ~/.bashrc` to
> apply the changes to your current terminal session.

## Install Java Extension for Zed

Open Zed, go to the Extensions view and install the `Java` extension.

https://zed.dev/extensions/java

This extension provides support for Java language features, including syntax highlighting
and LSP.

## Create a Project

As I'm not a regular Java user, I will create a simple project using Maven through Docker.

For this I ran the following command to create an _alias_ for `mvn` so that it runs inside
the container but mounts the current directory as a volume, so that the files are created in
the host machine.

```bash
alias mvn='docker run -it --rm -v "$(pwd)":/usr/src/app -w /usr/src/app maven:3.9.11 mvn'
```

Finally I created the project as:

```bash
mvn \
    archetype:generate \
    -DgroupId=com.company.app \
    -DartifactId=hello-world \
    -DarchetypeArtifactId=maven-archetype-quickstart \
    -DarchetypeVersion=1.5 \
    -DinteractiveMode=false
```

You can build the project with Maven as well:

```bash
mvn package
```

And finally execute it with:

```bash
mvn exec:java -Dexec.mainClass="com.company.app.App"
```

Its important to note that all of this Java code is actually running inside the Docker container,
you might find issues with running GUI applications, but for console applications this works
fine.

## Conclusion

Now you have a working Java environment on macOS using Zed as your main IDE. You can
create, build and run Java projects easily.
