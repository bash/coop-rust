# coop-rust

[![Build Status](https://travis-ci.org/bash/coop-rust.svg?branch=master)](https://travis-ci.org/bash/coop-rust)

Rust client for [STJEREM/coop](https://github.com/STJEREM/coop).


## Installation

### OSX

#### Installation

```bash
brew tap bash/homebrew-coop
brew install coop-rust
````

#### Bash Completion

Add the following to your `~/.bash_profile`:

```
. /usr/local/etc/bash_completion.d/coop-completion.bash
```


## Usage

```
Usage: coop [COMMAND]

Available Subcommands:
 - menus [LOCATION]     -  Shows menus for a location
 - locations            -  Lists all available locations
 - dish-stats           -  Shows stats dishes
```
