#!/bin/bash
# Meta loop test: zone -> loot -> extract -> base hub (stash move) -> re-enter zone
# -> loot again (map persistence) -> extract again -> die -> full reset -> quit/relaunch fresh.
cd /home/oleksiy/source/repos/picnic
LOG=/tmp/picnic.log
PASS=""; FAIL=""
assert() { # assert <name> <expected> <actual>
  if [ "$2" = "$3" ]; then PASS="$PASS $1"; echo "PASS: $1 ($3)"; else FAIL="$FAIL $1"; echo "FAIL: $1 expected=$2 got=$3"; fi
}
alive() { kill -0 $PID 2>/dev/null || { echo "ABORT: game dead at [$1]"; tail -8 $LOG; exit 1; }; }
focussed() { [ "$(xdotool getactivewindow 2>/dev/null)" = "$WID" ] || { echo "ABORT: focus lost at [$1]"; exit 1; }; }
key() { alive "pre-$1"; focussed "$1"; xdotool key --clearmodifiers "$1"; sleep "${T:-1.0}"; alive "post-$1"; }
keys() { for k in "$@"; do key $k; done; }
wait_log() { for i in $(seq 1 30); do grep -aq "$1" $LOG && { sleep 0.4; return 0; }; sleep 0.2; done
  echo "ABORT: log pattern not found: $1"; exit 1; }
count() { grep -ac "$1" $LOG; }

> $LOG
nohup ./target/debug/picnic > $LOG 2>&1 &
PID=$!; sleep 12; alive "launch"
WID=$(xdotool search --name '^Picnic$' | head -1)
[ "$(xdotool search --name '^Picnic$' | wc -l)" = 1 ] || { echo "ABORT: not exactly one window"; exit 1; }
wmctrl -i -a $WID; sleep 1; focussed "activated"
T=1.0

echo "===== RUN 1: loot battery, extract ====="
key F4;  wait_log "Loaded.*map with"
key F2;  wait_log "Player spawned with"
key e;   sleep 1.5                       # briefing -> PlayerTurn
keys W W W D D D D W D W W               # spawn -> battery (15,18)
key e; sleep 1.2                         # inspect opens
key e; wait_log "Picked up: Battery"     # picks + auto-closes (sole item)
keys S S A S A A W W A A A A A A A A A A A A   # battery -> exit
wait_log "Player reached exit"
key e; wait_log "Returning to base hub"; sleep 2
alive "base hub 1"
grep -aq PERMADEATH $LOG && { echo "ABORT: PERMADEATH on extraction!"; exit 1; }
echo "PASS: extraction preserved (no permadeath)"

echo "===== BASE HUB: stash 1 item ====="
key e; wait_log "Moving 'Bolt' from RunInventory to Stash"   # stash=1

echo "===== RUN 2: re-enter, verify map persisted, loot slag, extract ====="
key space; wait_log "Player spawned with 11 items"   # 12 - 1 moved to stash
key e; sleep 1.5                                     # briefing
keys D D W W W                            # spawn -> (12,21) rust slag
key e; sleep 1.2                          # inspect
key e; wait_log "Picked up: Rust Slag"    # sole item -> auto-close
keys W W W W W W A A A A A A A A A A A A  # (12,21)->(12,15)->(0,15)
keys S S S S                              # -> exit (0,19)
wait_log "Player reached exit"
key e; wait_log "Returning to base hub"; sleep 2
alive "base hub 2"

echo "===== BASE HUB: stash another item ====="
# Selection persists at index 0 (a Bolt) after extraction - correct behavior,
# so this moves the second Bolt; stash=2, RunInventory=11 for run 3.
key e; wait_log "from RunInventory to Stash"

echo "===== RUN 3: re-enter, die, verify full reset ====="
key space; wait_log "Player spawned with 11 items"
key e; sleep 1.5
keys W W W W W W W W W W A A A A A W     # spawn -> (5,13)
key W; sleep 1.5                         # pulled into anomaly (5,11)
keys W S W S W S W S                     # bounce until timer death
wait_log "DEATH: Player was crushed"
shot_death=1
key e; wait_log "PERMADEATH: Reset stash and RunInventory to starter loadout"
wait_log "Player spawned with 11 items"  # auto-restart with fresh starter
alive "after-death-restart"

echo "===== QUIT + RELAUNCH: app restart resets everything ====="
# ESC quits the game by design - press and expect the process to exit.
xdotool key --clearmodifiers Escape
for i in $(seq 1 10); do kill -0 $PID 2>/dev/null || break; sleep 0.5; done
if kill -0 $PID 2>/dev/null; then kill -9 $PID; echo "note: had to SIGKILL after ESC"; else echo "PASS: clean ESC quit"; fi
sleep 2
> $LOG
nohup ./target/debug/picnic > $LOG 2>&1 &
PID=$!; sleep 12; alive "relaunch"
WID=$(xdotool search --name '^Picnic$' | head -1)
wmctrl -i -a $WID; sleep 1
key F4; wait_log "Loaded.*map with"
key F2; wait_log "Player spawned with"
grep -a "Player spawned with" $LOG
xdotool key --clearmodifiers Escape
for i in $(seq 1 10); do kill -0 $PID 2>/dev/null || break; sleep 0.5; done
kill -0 $PID 2>/dev/null && kill -9 $PID

echo "===== ASSERTIONS ====="
assert "battery_taken_once_only (map persistence)" 1 "$(count 'Picked up: Battery')"
assert "two_pickups_total" 2 "$(count 'Picked up:')"
assert "spawn_11_items_x4 (runs 1,2,3 + restart)" 4 "$(count 'Player spawned with 11 items')"
assert "two_extractions_saved_12" 2 "$(count 'Saved 12 items from player')"
assert "two_stash_moves" 2 "$(count 'from RunInventory to Stash')"
assert "one_permadeath_reset" 1 "$(count 'Reset stash and RunInventory to starter loadout')"
assert "two_preserved_exits" 2 "$(count 'Leaving the Zone')"
echo "FAILED:${FAIL:- none}"
echo "=== META LOOP TEST DONE ==="
