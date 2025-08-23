---
title: "Sandbox Claude Code with Docker"
description: "\"Sandbox\" Claude Code using Docker to secure your host system from prompt injection and arbitrary bash commands."
categories: [ai, claude, copilot, agents, docker]
icon: rust
date: 2025-08-22
preview_image_url: "https://images.unsplash.com/photo-1506322526487-d8a94af63688?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&w=640"
published: true
---

As I adopt more and more AI tools into my daily workflow, one of the concerns that arises is the potential for prompt injection attacks and the execution of arbitrary bash commands on the system.

I try to read every generated code before accepting it into my project and also before executing. There are background bash commands that are also running to retrieve context on the agent side, which is harder to monitor.

So, I decided to take action and extend my productivity by allowing Claude to move faster without performing any potential harm.

## Using Docker as a "Sandbox"

Docker is a great tool to isolate processes, network access, and filesystem access. It allows you to use different OS environments, limit hardware resources, and clone your project files in a volume, allowing you to perform changes on files without accessing the actual host files.

It is perfect for creating an isolated environment for Claude Code to run and execute bash commands and perform file creation and modification inside the constrained filesystem.

So, I came up with the following `Dockerfile` to create a sandbox environment for Claude Code:

```dockerfile
FROM alpine:3

RUN apk add -f -u \
    bat \
    bash \
    ca-certificates \
    curl \
    fd-find \
    git \
    jq \
    npm \
    ripgrep \
    sudo \
    tree

RUN npm install -g @anthropic-ai/claude-code

RUN mkdir -p /workspaces/app

WORKDIR /workspaces/app
```

This Dockerfile installs Claude Code using the standard approach in an Alpine Linux container.

To run it, I use a `docker-compose.yml` file to allow me to configure volumes and run the container easily.

```yaml
services:
  claude:
    build:
      dockerfile: Dockerfile
    volumes:
      - ../:/workspaces/app
    command: sleep infinity
```

In my repo [DeckMaster](https://github.com/LeoBorai/DeckMaster), where I'm writing a solution to manage Magic: The Gathering cards and decks, I'm running this development environment for Claude, where I can perform my development tasks via the container runtime.
