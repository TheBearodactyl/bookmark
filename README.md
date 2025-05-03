# Command-Line Help for `bookmark`

This document contains the help content for the `bookmark` command-line program.

**Command Overview:**

* [`bookmark`↴](#bookmark)
* [`bookmark init`↴](#bookmark-init)
* [`bookmark list`↴](#bookmark-list)
* [`bookmark open`↴](#bookmark-open)
* [`bookmark add`↴](#bookmark-add)
* [`bookmark remove`↴](#bookmark-remove)

## `bookmark`

**Usage:** `bookmark <COMMAND>`

###### **Subcommands:**

* `init` — Initialize the config file
* `list` — List the current bookmarks and their data
* `open` — Open a specific bookmark's path with your system default
* `add` — Add a bookmark
* `remove` — Remove a bookmark



## `bookmark init`

Initialize the config file

**Usage:** `bookmark init`



## `bookmark list`

List the current bookmarks and their data

**Usage:** `bookmark list`



## `bookmark open`

Open a specific bookmark's path with your system default

**Usage:** `bookmark open <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the bookmark



## `bookmark add`

Add a bookmark

**Usage:** `bookmark add --name <NAME> --path <PATH> --description <DESCRIPTION>`

###### **Options:**

* `-n`, `--name <NAME>` — The name of the bookmark
* `-p`, `--path <PATH>` — The path to the bookmarked file/folder
* `-d`, `--description <DESCRIPTION>` — The description for the bookmark



## `bookmark remove`

Remove a bookmark

**Usage:** `bookmark remove <NAME>`

###### **Arguments:**

* `<NAME>` — The name of the bookmark



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
