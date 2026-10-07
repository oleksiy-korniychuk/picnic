# Validation harness

Scripted playtest/validation for Picnic. All scripts inject keys with
`xdotool` (game window must be focused on DISPLAY=:0), poll the game log at
`/tmp/picnic.log`, and capture window-relative screenshots via
`xwd_grab.py` (position-independent; ImageGrab crops break when the window
moves or resizes on screen).

| Script | What it does |
|--------|--------------|
| `meta_loop.sh` | Full meta loop: 3 runs - loot, extract, stash, re-enter, map-persistence check, death reset, relaunch-fresh check |
| `ui_final.sh`  | UI/UX validation: key bar contexts, quit-confirm modal (F10 geometry dump), base hub footers, moves, small-window fit |
| `xwd_grab.py`  | Window capture via `xwd` (see header for usage) |
| `grab_window.py` | Screen-crop capture via xwininfo geometry (kept for reference; superseded by xwd_grab) |

Note: these scripts take over the keyboard for a few minutes - don't type
while one is running. They also `pkill -x picnic` at startup.
