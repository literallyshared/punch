# Punch
Example layout of the time file:

~/.punch/2025/August/2025-08-31
```
07:50 - 12:00: On-boarding
12:30 - 16:05: On-boarding
```

## Installation

```
cargo build --release
sudo ./install.sh
```

## Usage

Start your day:
```
punch in
```
Finish a task:
```
punch out "Working on something"
```
Now you have a time card in ~/.punch/<YEAR>/<MONTH>/<YY-mm-dd> with a time registered as "Working on something" between the two time stamps punching in and out.

```
punch out
```
Punching out without an argument warns you and gives you a placeholder activity. You should use the edit functionality to properly reflect what you were working on:

```
punch edit
```
Launches your $EDITOR and lets you edit the current day.
```
punch edit 25-08-04
```
Providing a date to the edit command lets you edit a specific day.

```
punch report
```
Prints a report for the current day.

```
punch report 25-08-04
```
Prints a report for the given day.


# Roadmap
While it is a simple program, I find it easy to use in my day to day and I would like to see if more people can find it useful. If so I see a few features I would like to add:
* Support for multiple date formats.
