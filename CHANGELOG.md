# Changelog

## v0.0.4 (2023-10-31)

## Features
- introduced subcommand "start" and "log"
- added option to clear log
- stop timer and record elapsed time by pressing "s"

## Fixed
- add empty log to json if file created (or empty)
- convert terminal size to u32 in function
- correctly handle opening data file
- moved file-handling functions to utils


## v0.0.3 (2023-10-25)

## Features
- adds start and end time to entry

## Fixed
- handle data.json file creation if not found


## v0.0.2 (2023-10-24)

### Features
- parse input into json entry
- add completed entry to json file
- read and write json database file


## v0.0.1 (2023-10-20)

### Features
- reads task and duration input
- starts timer visualized by bar
