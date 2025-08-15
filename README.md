# Punch

Punch is a punch clock for your terminal. Punch in. Punch out. Track your time for your biling, time reporting, etc.
## Installation

```console
cargo build --release
sudo ./install.sh
```

## Usage

Start your day:

```console
punch in
```

Finish a task:

```console
punch out "Working on something"
```

Now you have a time card in ~/.punch/[YEAR]/[MONTH]/[YY-mm-dd] with a time registered as "Working on something" between the two time stamps punching in and out.

```console
punch out
```

Punching out without an argument warns you and gives you a placeholder activity. You should use the edit functionality to properly reflect what you were working on:

```console
punch edit
```

Launches your $EDITOR and lets you edit the current day.
Providing a date to the edit command lets you edit a specific day:

```console
punch edit 25-08-04
```

The report command can be used to see the time spent on different tasks during the day.

```console
punch report
```
Prints a report for the current day:
```console
--- 2025-8-31 ---
[Activity A]: 4 hours 21 minutes.
[Activity B]: 2 hours 30 minutes.

Total: 6 hours 51 minutes.
```

To see a report for a given day:
```console
punch report YY-mm-dd
```

Punch also keeps track of your on-going activity as well!
```console
[*Current*]: 2 hours 0 minutes.
[Activity A]: 4 hours 21 minutes.
[Activity B]: 2 hours 30 minutes.

Total: 8 hours 51 minutes.
```
