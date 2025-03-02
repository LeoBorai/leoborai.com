---
title: "How I setup my Python/Poetry Development Environment"
description: "A walkthrough on setting up Poetry development environment for Unix systems"
categories: [python, poetry, macos, setup, homebrew, development]
icon: python
date: 2022-08-06
preview_image_url: "https://images.unsplash.com/photo-1538439907460-1596cafd4eff?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2912&q=10"
published: true
---

Recently I started to work with [Saleor][1]. Saleor is a headless e-commerce
platform built with Python, specifically Django, which in my opinion it's a
great HTTP Server Framework. This experience took me back to Python development.

Python is a very intuitive, easy to learn and a very enjoyable programming
language. It allows you to express logic on a simple and minimalistic way,
its dynamic, interpreted and it has a huge community, with solutions for almost
anything.

After having some time without using Python on a daily basis, I had to install
it again on my system, so I decided to document it along the way.

## Tools Involved

For the development environment setup, I will use a Version Manager and a
Package manager. If you are not familiarized with one of these concepts, I
recommend you to read through the next two sections. Otherwise feel free to
move into [Setup](#setup).

### Version Manager

For Python (and actually other programming languages), I like to use what is
called _Version Manager_. This is very helpful when your work on projects
on which Programmning Languages versions are different.

For instance, Ruby has [RVM][2], for NodeJS we have [NVM][3], and for Python
we have [pyenv][4]. I recommend you to install a Version Manager, so you can
switch between Python versions when working on different projects.

### Package Manager

Back on 2012 when I meet Python ⏤ reading [Head First Python][5], great book
for Software Development beginners ⏤ I remember using `pip` to install packages
from the PyPi package registry. Pip works just fine if you work on small
projects. But as your pojects becomes complex, you are likely to move into other
solutions, I remember using [Pipenv][6] around 2019 to work on a changelog
generator.

The experience with [Pipenv][6] was smooth comming from the de-facto package
manager, having a virtual environment manager out of the box was great. Another
good feature was the fact of having the `requirements.txt` file replaced by
the `Pipfile` which uses the TOML format ⏤ similar to how Rust does with
`Cargo.toml` file ⏤ a joyful experience.

Coming back to Python on 2022, and with Python 3.10 which introduces enhanced
support for type definitons, I was introduced to [Poetry][7].

Poetry not only allows you to manage dependecies for your project, but also
allows you to build your project, publish to PyPi and helps your keep dependencies up to date.

## Setup

Now that you know how our setup is composed, lets move into actually installing
this software in our machines. I love to install system dependencies with
[Homebrew][8], it makes very straightforward adding packages to your system
and its very easy to track where they come from.

> Homebrew is supported for macOS and Linux, for Windows [Chocolatey][9] is the most common option.

If you dont have Homebrew installed I recommend you to do it by running the
following command:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

> Source: https://brew.sh/

If you are running it for the first time, it may take a couple minutes to
finish.

### Installing Pyenv

In order to install Pyenv with Homebrew, you must run the following command:

```bash
brew install pyenv
```

This will install Pyenv in your system.

Then make sure Pyenv is installed by running the `pyenv` command, you will see
something similar to:

```
➜  ~ pyenv
pyenv 2.3.3
Usage: pyenv <command> [<args>]

Some useful pyenv commands are:
   --version   Display the version of pyenv
   commands    List all available pyenv commands
   exec        Run an executable with the selected Python version
   global      Set or show the global Python version(s)
   help        Display help for a command
   hooks       List hook scripts for a given pyenv command
   init        Configure the shell environment for pyenv
   install     Install a Python version using python-build
   local       Set or show the local application-specific Python version(s)
   prefix      Display prefixes for Python versions
   rehash      Rehash pyenv shims (run this after installing executables)
   root        Display the root directory where versions and shims are kept
   shell       Set or show the shell-specific Python version
   shims       List existing pyenv shims
   uninstall   Uninstall a specific Python version
   version     Show the current Python version(s) and its origin
   version-file   Detect the file that sets the current pyenv version
   version-name   Show the current Python version
   version-origin   Explain how the current Python version is set
   versions    List all Python versions available to pyenv
   whence      List all Python versions that contain the given executable
   which       Display the full path to an executable

See `pyenv help <command>' for information on a specific command.
For full documentation, see: https://github.com/pyenv/pyenv#readme
```

### Installing Poetry

With Pyenv installed we can move into Poetry.

```bash
brew install poetry
```

Poetry will not use your system's Python version, if you read through the
logs from `brew install poetry`, you will find the following piece printed:

```
Installing poetry dependency: python@3.10
```

Run the Poetry command to make sure it is installed with success.

```
➜  ~ poetry
Poetry version 1.1.14

USAGE
  poetry [-h] [-q] [-v [<...>]] [-V] [--ansi] [--no-ansi] [-n] <command>
         [<arg1>] ... [<argN>]

ARGUMENTS
  <command>              The command to execute
  <arg>                  The arguments of the command

GLOBAL OPTIONS
  -h (--help)            Display this help message
  -q (--quiet)           Do not output any message
  -v (--verbose)         Increase the verbosity of messages: "-v" for normal
                         output, "-vv" for more verbose output and "-vvv" for
                         debug
  -V (--version)         Display this application version
  --ansi                 Force ANSI output
  --no-ansi              Disable ANSI output
  -n (--no-interaction)  Do not ask any interactive question

AVAILABLE COMMANDS
  about                  Shows information about Poetry.
  add                    Adds a new dependency to pyproject.toml.
  build                  Builds a package, as a tarball and a wheel by default.
  cache                  Interact with Poetry's cache
  check                  Checks the validity of the
                         pyproject.toml file.
  config                 Manages configuration settings.
  debug                  Debug various elements of Poetry.
  env                    Interact with Poetry's project environments.
  export                 Exports the lock file to alternative formats.
  help                   Display the manual of a command
  init                   Creates a basic pyproject.toml file in the
                         current directory.
  install                Installs the project dependencies.
  lock                   Locks the project dependencies.
  new                    Creates a new Python project at <path>.
  publish                Publishes a package to a remote repository.
  remove                 Removes a package from the project dependencies.
  run                    Runs a command in the appropriate environment.
  search                 Searches for packages on remote repositories.
  self                   Interact with Poetry directly.
  shell                  Spawns a shell within the virtual environment.
  show                   Shows information about packages.
  update                 Update the dependencies as according to the
                         pyproject.toml file.
  version                Shows the version of the project or bumps it when a
                         valid bump rule is provided.
```

## Conclusion

With Poetry and Pyenv installed _We are ready to roll!_. I recommend you to
read through Poetry documentation to learn about it's capabilities, I really
enjoy unsing it. Thanks for reading if got this far! If you have any issues
while following through this guide, please open a GitHub issue in my personal
website repository [here][10].

Have a great time hacking with Python!

[1]: https://github.com/saleor/saleor
[2]: https://rvm.io
[3]: https://github.com/nvm-sh/nvm
[4]: https://github.com/pyenv/pyenv
[5]: https://www.goodreads.com/book/show/8933914-head-first-python
[6]: https://pipenv.pypa.io/en/latest/
[7]: https://python-poetry.org
[8]: https://brew.sh/
[9]: https://chocolatey.org
[10]: https://github.com/LeoBorai/leoborai.com
