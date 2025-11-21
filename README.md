# Parrot 🦜

Parrot is a CLI tool that allows you to record and replay your terminal sessions with ease. It can
capture commands that you would like to reuse later, making it a great tool for automating
repetitive tasks.

Justfiles, Makefiles, package.json files etc. solve part of this problem, but they are project
specific. Parrot allows you to create reusable command sequences that you find yourself repeating
and can be used anywhere, without being tied to a specific project.

## Features

- Record commands interactively or through CLI arguments
- Save recorded commands to a file for later use
- Replay recorded commands in the terminal
- Simple and intuitive command-line interface

## Installation

Make sure Parrot is in your system's PATH. After this you need to set up shell integration for
your shell. Below are instructions for Fish shell.

### Fish Shell Integration

Add the following line to your `~/.config/fish/config.fish` file:

```fish
parrot init fish | source
```

This will enable Parrot's shell integration for Fish shell, which is necessary for recording and
replaying commands.

## Examples

Record commands non-interactively:

```fish
# Record a sequence of commands

parrot record start -t nameofrecording
parrot record add 'ls -la'
parrot record add 'whoami'
parrot record stop

# Replay recorded commands
parrot replay -t nameofrecording
```

Record commands interactively:

```fish
parrot record interactive -t nameofrecording
# Your editor will open, allowing you to enter commands line by line.
# Save and exit the editor to finish recording.

# Replay recorded commands
parrot replay -t nameofrecording
```

Example showing showcasing shell integration:

```fish
set -x number 5
parrot record start -t mysession
parrot record add 'echo "The number is $number"'
parrot record stop

parrot replay -t mysession
# Output: The number is 5

set -x number 10
parrot replay -t mysession
# Output: The number is 10
```

## Todo

- Add support for other shells than Fish (e.g., Bash, Zsh)
- Implement a way to edit recorded commands interactively
- Implement a way to delete recordings by tag
