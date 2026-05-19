# pomo

A minimal Pomodoro timer for the terminal. I made this on the plane.

## Usage

```
pomo [work_mins] [break_mins]
```

Defaults to 60 minute work sessions and 10 minute breaks.

```
pomo          # 60min work / 10min break
pomo 25 5     # 25min work / 5min break
```

The timer counts up, showing the current phase and round:

```
[Round 1] STUDY 00:24:07
```

A terminal bell fires at the end of each work and break phase.

## Build

```
cargo build --release
```
