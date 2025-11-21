# Parrot 🦜

Parrot is a CLI tool that allows you to record and replay your terminal sessions with ease. It can
capture commands that you would like to reuse later, making it a great tool for automating
repetitive tasks.

## Features

- Record commands interactively or through CLI arguments
- Save recorded commands to a file for later use
- Replay recorded commands in the terminal
- Simple and intuitive command-line interface

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
