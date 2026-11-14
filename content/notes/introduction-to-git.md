---
title: "Introduction to Git: Repositories, Commits & Branches"
description: "A primer on Git repositories, commits & branches"
categories: [git, github, unix, windows, repositories, commits, branches]
icon: git
date: 2021-10-28
preview_image_url: "https://images.unsplash.com/photo-1519389950473-47ba0277781c?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1470&q=80"
published: true
---

If you are learning about software development, you may heard about Git and
GitHub many times. Both are tools used on a daily basis by most software
developers.

Git is a _version control system_ software used to manage versions of a set of
files and directories, using Git terms, this set of _files and directories_ is
actually called a "Repository".

> Before reading further it's important to note that basic usage of the terminal is required. You will need to create files, delete files, as well as directories and also walk through directories.

## Installing Git

Git must be installed in your system to go through, there's
many ways to install Git, it may vary depending on which operative system you
are using and what's your preferred approach.

Official binaries to install Git are available [here](https://git-scm.com/downloads).
Other approaches are enumerated below for most popular operative systems.

**Linux**

You can install Git in your Linux system by running the following commands:

```bash
sudo apt-get update && sudo apt-get install git
```

**macOS**

Git is installed along other macOS development software when installing
Xcode Command Line Tools, you must execute `xcode-select --install` in the
terminal.

Another approach would be using Homebrew, if you are a Homebrew user you can
install Git by executing `brew install git` in the macOS terminal.

**Windows**

You can use [Git for Windows](https://gitforwindows.org/) if you are not using
WSL or WSL2.

If you are using WSL/WSL2, you may follow the same approach as for Linux.

## Checking if Git is available

After installing Git you will want to check if the installation was successful.
Open the terminal and execute the following command.

```bash
git --version
```

You must get a version output similar to this:

```bash
git version 2.33.1
```

## Creating your first repository

Let's create a repository, I will be using the following path:

```
~/Desktop/libmath
```

> If you want to followup with the same directory, just open the terminal and run the following command `mkdir ~/Desktop/libmath && cd ~/Desktop/libmath/`

Open the terminal and step on the directory you just created for the purposes of
this tutorial, then initialize the repository executing the following command:

```bash
git init
```

The `init` subcommand, will initialize a Git repository if none is already
initialized in the directory.

If the command ran succesfully you may have the following output:

```
Initialized empty Git repository in /home/esteban/Desktop/libmath/.git/
```

¡You have just created your first repository, congratulations!

## Creating and tracking a project

Now that we have a brand new repository, we are able to begin with our
development tasks and make sure we track every change in our project.

We will create a very simple math library and we will add features to it with
the purpose of creating a "checkpoint" (actually they are called _commit_ in
Git-land) whenever we introduce features or bug fixes to our library.

Create a new file in our repository:

```bash
touch lib.js
```

Then open the file and copy the following JavaScript code into it:

```js
function sum(a, b) {
  return a + b;
}
```

We have added a new function to our math library, the `sum`
function. Let's save the file and create a commit for this
change so we track it in our repository.

First, let's check our libmath's repository history, open
the terminal, step into the libmath directory and run `git log`.

You may get a message similar to this one:

```
fatal: your current branch 'main' does not have any commits yet
```

This is because we haven't created any commits already.

> You may have a different branch name, such as `master`. This is because some Git versions make use of the `master` branch given that it has been the default name for a while. Newer versions are starting to move to the `main` branch. For the purposes of this tutorial any of these are just fine, just keep in mind that you have to stick to one name for further reading. If you want to change the name of the branch create a new one by running: `git checkout -b my-branch-name`.

In order to track changes when using Git, you must first choose which files you
want to track (add the file to the staging area), and then create a commit for
such files.

Add the `lib.js` file to the staging area by running `git add lib.js`. You can
check which files are added to the staging area by running:

```bash
git diff --name-only --cached
```

Now lets create a commit, a commit is similar to a game's checkpoint, it stores
the current state of the files and directories in your repository, this is
also known as a snapshot.

To create a commit you must execute the `git commit` command and provide a
message which describes in a sentence of no more than 50 characters what is
this commit about.

Personally I'm a fan of the [Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0/) specification.

The following command creates a commit and also provides the message for the
commit in a single line:

```bash
git commit -m 'feat: implement the `sum` function'
```

> Commit messages are required, you must provide one everytime you create a new commit

An output similar to this one must show up:

```
[main (root-commit) 28bcd7d] feat: implement the `sum` function
 1 file changed, 3 insertions(+)
 create mode 100644 lib.js
```

¡You have just created your first commit!

Now our libmath project has a new commit which we can inspect in the `git log`!
Check it out:

```
commit 28bcd7d811e0da14acb3fcf1a0482603e0bc4238 (HEAD -> main)
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:00:08 2021 -0300

    feat: implement the `sum` function
```

Remember the `git log` is basically the history of our repository, here you will
find every commit.

Now that you know how to create commits, let's add more features to our math
library.

Paste the following code in the `lib.js` file and then save the file.

```js
function substract(a, b) {
  return a - b;
}

function multiply(a, b) {
  return a * b;
}

function divide(a, b) {
  return a / b;
}
```

The math library now supports substraction, multiplication and division.

Let's create another commit!

```bash
# add all files to the stage
git add .

# commit staged files
git commit -m 'feat: impl `substract`, `multiply` and `divide`'
```

You may have an output similar to this:

```
[main de154de] feat: impl `substract`, `multiply` and `divide`
 1 file changed, 12 insertions(+)
```

This is our commit summary, which holds the same details as in our previous
commit:

- `main`: The current branch
- `de154de`: Commit hash
- `feat: impl ...`: Commit message
- `1 file changed`: Lines changed
- `12 insertions(+)`: Lines added

Our `git log` will now hold entries, one per commit:

```bash
commit de154de77e4901f5c8a946080fce47bb083cdd93 (HEAD -> main)
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:07:37 2021 -0300

    feat: impl `substract`, `multiply` and `divide`

commit 28bcd7d811e0da14acb3fcf1a0482603e0bc4238
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:00:08 2021 -0300

    feat: implement the `sum` function
```

## Enter the `git branch`

Now we know that `git log` shows us the history of commits we have
created in our project. If you check on other projects such as the ones hosted
in GitHub you will see that the `git log` output may be huge.

Check it out for repositories like Rust!

[rust-lang/rust commit history](https://github.com/rust-lang/rust/commits/master).

There's many participants, theres complex messages, theres lots of hashes!

When working in software development teams you'll likely to create a branch
every time you want to suggest a change to the project. Using simple terms we
can say that a branch is a copy of the repository history that you keep for your
own purposes.

Let's create a new branch to introduce a fix on our `divide` function, we will
throw an error when `b` (the divisor) is `0`.

Create a new branch using the following command:

```bash
git checkout -b fix/throw-on-divide-by-0
```

You can check the current active branch by running the `git branch` command, the
output must be similar to this:

```bash
* fix/throw-on-divide-by-0
  main
```

Given that an asterisk is next to the `fix/throw-on-divide-by-0` branch name,
we can tell we are in such branch.

We can also note that `main` is listed as another available branch, you can
switch between branches using `git checkout` followed by the branch name.

```bash
git checkout main
```

### Introducing the fix

Make sure `fix/throw-on-divide-by-0` is your active branch, then open the `lib.js`
file and proceed to replace our `divide` function with the following:

```js
function divide(a, b) {
  if (b === 0) {
    throw new Error('Dividing by 0 is not allowed');
  }

  return a / b;
}
```

Then let's commit our change and check `git log`.

```bash
# add all the files to the stage
git add .

# commit the staged files
git commit -m 'fix(division): throw on divider equals 0'

[fix/throw-on-divide-by-0 5925cf6] fix(division): throw on divider equals 0
 1 file changed, 4 insertions(+)

## check on the `git log` output
git log

commit 5925cf6545ceb4843103d7c1f9339c385c08da4f (HEAD -> fix/throw-on-divide-by-0)
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:34:45 2021 -0300

    fix(division): throw on divider equals 0

commit de154de77e4901f5c8a946080fce47bb083cdd93 (main)
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:07:37 2021 -0300

    feat: impl `substract`, `multiply` and `divide`

commit 28bcd7d811e0da14acb3fcf1a0482603e0bc4238
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:00:08 2021 -0300

    feat: implement the `sum` function
```

Did you noticed those parentheses next to the last two hashes in our `git log`?
Those are the name of the branches!

Our latest commit on `main` was "feat: impl `substract`, `multiply` and `divide`"
which in this log holds the hash: `de154de77e4901f5c8a946080fce47bb083cdd93`,
then we have our latest commit, which only exists for `fix/throw-on-divide-by-0`.

You can change the branch back to `main` and run `git log` again and you will
see that there's only 2 commits so far. If you also take a look to the `lib.js`
file you will see that our `if` clause is gone!

You can go back to `fix/throw-on-divide-by-0` and you will notice that our
"fix(division): throw on divider equals 0" commit is back and you will also see
that our `if` clause is back.

This is what Git is for, Git takes care of handling different versions for our
work, and helps keeping track of important changes for our projects.

Now we have 2 versions of our projects living in different branches, one that
throws when the divisor equals to 0 and another that doesn't.

### Consolidating versions

When we decide that our work is complete and that we want to ship the changes to
the `main` branch, then it's time to merge!

There's many ways you can merge two branches using Git, but to keep things
simple I will use `git merge`.

First you must step on the branch you want to merge from, this means the branch
you want to bring changes to.

So if we want to bring changes commited on `fix/throw-on-divide-by-0` to `main`,
we have to "step" on `main` and then execute `git merge fix/throw-on-divide-by-0`.

```bash
# change the branch to `main`
git checkout main

# merge branch against "fix/throw-on-divide-by-0"
git merge fix/throw-on-divide-by-0
Updating de154de..5925cf6
Fast-forward
 lib.js | 4 ++++

# make sure commits are now on "git log"
git log

commit 5925cf6545ceb4843103d7c1f9339c385c08da4f (HEAD -> main, fix/throw-on-divide-by-0)
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:34:45 2021 -0300

    fix(division): throw on divider equals 0

commit de154de77e4901f5c8a946080fce47bb083cdd93 (main)
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:07:37 2021 -0300

    feat: impl `substract`, `multiply` and `divide`

commit 28bcd7d811e0da14acb3fcf1a0482603e0bc4238
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:00:08 2021 -0300

    feat: implement the `sum` function
```

¡Awesome! We have just updated our default branch (`main` in my case), to have
new commits from `fix/throw-on-divide-by-0`.

Note that the latest commit is now holded by both `main` and `fix/throw-on-divide-by-0`.

```
commit 5925cf6545ceb4843103d7c1f9339c385c08da4f (HEAD -> main, fix/throw-on-divide-by-0)
Author: Leo Borai <estebanborai@gmail.com>
Date:   Thu Oct 28 19:34:45 2021 -0300

    fix(division): throw on divider equals 0
```

Is a good practice to remove branches from Git when we no longer need them, to
do that you must run `git branch -D` followed by the name of the branch you want
to delete:

```bash
git branch -D fix/throw-on-divide-by-0
```

You will have an output similar to:

```
Deleted branch fix/throw-on-divide-by-0 (was 5925cf6).
```

## Conclusion

That's everything for this note! I hope you really liked it and learned
something new. If you find any typo or want to contribute please don't hesitate
on opening a Pull Request on GitHub!
