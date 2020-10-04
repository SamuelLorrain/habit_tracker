# Habit tracker

A simple habit tracker,
I created it as a first Rust project

Nothing fancy but it works great.

## Usage

```
habit_tracker
A simple habit tracker with stats

USAGE:
        habit_tracker [OPTIONS]

OPTIONS:
        --list                                     List all current habits
        --today                                    List all habits todo today
        --done <NAME>                              Mark a habit has done (if it is d
ue today)
        --new <NAME>                               Create a new habit
        --freq <NAME> <FREQ> <FREQ_UNIT> [OPTIONS] Change frequency of the habit
        --time <NAME> <TIME>                       Change time of the habit
        --begin <NAME> <DATE>                      Change begin date of the habit (d
efault: today)
        --end  <NAME> <TIME> <TIME_TYPE>           Add endtime for the habit (defaul
t: none)
        --meta <NAME> <META>                       Add metadata to the habit
        --delete  <NAME>                           Delete habit from the database
        --history <NAME>                           History for the given habits
        --missing <NAME>                           List every day the habit has been
 missed

        --help                                     Show help
```

## Todo

- Add some stats
- Add colors
- Some options are missing
- Better help
- Archives
- ... 

