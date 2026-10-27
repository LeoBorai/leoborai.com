---
title: "Use Google Chrome Incognito Mode as default mode on macOS"
description: "Walkthrough on how to set Google Chrome Incognito Mode as default mode on macOS"
categories: [chrome, browser, development, macos, setup]
date: 2024-05-27
icon: dev
preview_image_url: "/images/preview/use-chrome-incognito-as-default.png"
published: true
---

When working on Web Development projects, I often find myself needing to test the website in Incognito Mode.

By default I use Firefox and Google Chrome as browsers for testing, when it comes to Firefox setting up a default
"Incognito Mode" is pretty straightforward, I just disable _History_ and _Cookies_ in the settings and I'm good to go.

However, when it comes to Google Chrome, there is no such setting, so I rely on openning Google Chrome and then
switch it to Incognito Mode manually, which is a bit annoying. I wanted to have Google Chrome open in Incognito Mode
by default.

## About `defaults` command

If you are a macOS user, you might have seen `plist` files, these files are used to store user preferences for
applications. You can use the `defaults` command to read, write, and delete these preferences.

Open your terminal and run `defaults` command to see the help message.

```
Command line interface to a user's defaults.
Syntax:

'defaults' [-currentHost | -host <hostname>] followed by one of the following:

  read                                 shows all defaults
  read <domain>                        shows defaults for given domain
  read <domain> <key>                  shows defaults for given domain, key

  read-type <domain> <key>             shows the type for the given domain, key

  write <domain> <domain_rep>          writes domain (overwrites existing)
  write <domain> <key> <value>         writes key for domain

  rename <domain> <old_key> <new_key>  renames old_key to new_key

  delete <domain>                      deletes domain
  delete <domain> <key>                deletes key in domain
  delete-all <domain>                  deletes the domain from all containers
  delete-all <domain> Key>             deletes key in domain from all containers

  import <domain> <path to plist>      writes the plist at path to domain
  import <domain> -                    writes a plist from stdin to domain
  export <domain> <path to plist>      saves domain as a binary plist to path
  export <domain> -                    writes domain as an xml plist to stdout
  domains                              lists all domains
  find <word>                          lists all entries containing word
  help                                 print this help

<domain> is ( <domain_name> | -app <application_name> | -globalDomain )
         or a path to a file omitting the '.plist' extension

<value> is one of:
  <value_rep>
  -string <string_value>
  -data <hex_digits>
  -int[eger] <integer_value>
  -float  <floating-point_value>
  -bool[ean] (true | false | yes | no)
  -date <date_rep>
  -array <value1> <value2> ...
  -array-add <value1> <value2> ...
  -dict <key1> <value1> <key2> <value2> ...
  -dict-add <key1> <value1> ...
```

Each application has its own `plist` file, for Google Chrome it's located at `~/Library/Preferences/com.google.Chrome.plist`.

## Setting Google Chrome to open in Incognito Mode by default

Knowing about the `defaults` command and the location of the `plist` file, we can now set Google Chrome to open
in Incognito Mode by default.

You can check for the current defaults on Google Chrome easily by running the following command:

```bash
defaults read com.google.chrome
```

Between many options (also known as _keys_) available, we are interested in the `IncognitoModeAvailability`
key, which is set to `0` by default.

The `IncognitoModeAvailability` key can have the following values:

Value | Description
--- | ---
0 | Incognito mode available.
1 | Incognito mode disabled.
2 | Incognito mode forced.

As I want to always use Incognito Mode, I will set the `IncognitoModeAvailability` key to `2`, to do this run the following command:

```bash
defaults write com.google.chrome IncognitoModeAvailability -integer 2
```

Now, when you open Google Chrome, it will open in Incognito Mode by default. In order for these changes to
take effect, you will need to restart your system!

> I know, its actually weird in macOS to restart the system for application settings to take effect
> but _Preferences_ are loaded at the time of login, so you need to restart your system to apply the changes.

## Setting up an _alias_ to have Google Chrome in incognito mode

If you don't want to restart your system, you can set up an _alias_ to open Google Chrome in Incognito Mode.

This approach will allow you to open Google Chrome in Incognito Mode from a bash alias without having to update
preferences settings.

Open your `.bashrc` or `.zshrc` file and add the following alias:

```bash
alias chrome="open -a /Applications/Google\\ Chrome.app --args --incognito"
```

Then run the following command to apply the changes:

```bash
source ~/.bashrc
```

or, if you are using ZSH:

```bash
source ~/.zshrc
```

Now you can open Google Chrome in Incognito Mode by running the following command:

```bash
chrome
```
