# Chomp
**Chomp** is a simple cli tool for tracking your daily calorie intake and weight.
It allows you to manage your data and synchronize it with data from other applications (currently only FitNotes)

# Notes
Assumes that use use unix system, so paths are unix-specific.
If you use windows and you would like me to support it just create an issue and I'll implement it.
I just didn't bother to do that without a reason

## Installation
```bash
git clone https://github.com/Neidz/chomp.git
cd chomp
go install
```

## Features and usage (--help command output)
```
Usage: chomp [command] [subcommand] [args...]

Available Commands:
  calories              Manage calorie intake. If no subcommand is provided, it will display the summary
    Subcommands:
      get               Get the calories for the selected date
      add <values...>   Add calorie entries for the selected date
      clear             Clear all calorie entries for the selected date
      fill              Fill remaining calories to reach the target for the selected date
      pop               Remove the last calorie entry for the selected date
      setTarget <value> Set a daily target for calorie intake

  weight                Manage weight tracking. If no subcommand is provided, it will display the summary
    Subcommands:
      get               Get the weight for the selected date
      set <value>       Set the weight for the selected date
      clear             Clear the weight entry for the selected date

  sync                  Synchronize current data with data from different applications
    Subcommands:
      fitnotes <path>   Safely add data from fitnotes app. This will not overwrite any of the existing data

  help                  Display this help message

Flags:
  --date                Set the date for the command execution (default is today, format is YYYY-MM-DD)
  --help                Display this help message
```
