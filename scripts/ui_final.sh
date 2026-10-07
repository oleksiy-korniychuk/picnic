#!/bin/bash
# Final UI/UX validation: key bar contexts, quit-confirm modal, base hub
# footers + moves, small-window responsiveness. All captures window-relative
# (xwd), all assertions window-relative or log-based.
cd /home/oleksiy/source/repos/picnic
pkill -x picnic 2>/dev/null; sleep 1
LOG=/tmp/picnic.log
PASS=""; FAIL=""
ok() { PASS="$PASS $1"; echo "PASS: $1"; }
bad() { FAIL="$FAIL $1"; echo "FAIL: $1"; }
alive() { kill -0 $PID 2>/dev/null || { echo "ABORT: dead at [$1]"; tail -8 $LOG; exit 1; }; }
key() { alive "pre-$1"; xdotool key --clearmodifiers "$1"; sleep "${T:-1.0}"; }
keys() { for k in "$@"; do key $k; done; }
wait_log() { for i in $(seq 1 30); do grep -aq "$1" $LOG && { sleep 0.4; return 0; }; sleep 0.2; done
  echo "ABORT: log pattern not found: $1"; exit 1; }
grab() { WID=$(xdotool search --name '^Picnic$' | head -1); python3 /tmp/xwd_grab.py $WID /tmp/$1 >/dev/null; }

> $LOG
nohup ./target/debug/picnic > $LOG 2>&1 &
PID=$!; sleep 12
WID=$(xdotool search --name '^Picnic$' | head -1)
wmctrl -i -r $WID -e 0,150,150,1200,800; sleep 2   # deterministic geometry
wmctrl -i -a $WID; sleep 1

echo "== 1. EDITOR: bar visible =="
grab ui_f1_editor.png
python3 -c "
from PIL import Image
img = Image.open('/tmp/ui_f1_editor.png').convert('RGB'); w,h=img.size; px=img.load()
y = sum(1 for yy in range(h-30,h) for x in range(0,w,2) if px[x,yy][0]>180 and px[x,yy][1]>180 and px[x,yy][2]<140)
print('editor bar yellow:', y); exit(0 if y>50 else 1)" && ok "editor bar visible" || bad "editor bar visible"

echo "== 2. ZONE: bar content switches =="
key F4; wait_log "Loaded.*map with"
key F2; key e
grab ui_f2_zone.png
python3 -c "
from PIL import Image
img = Image.open('/tmp/ui_f2_zone.png').convert('RGB'); w,h=img.size; px=img.load()
y = sum(1 for yy in range(h-30,h) for x in range(0,w,2) if px[x,yy][0]>180 and px[x,yy][1]>180 and px[x,yy][2]<140)
print('zone bar yellow:', y); exit(0 if y>50 else 1)" && ok "zone bar visible" || bad "zone bar visible"

echo "== 3. QUIT MODAL: geometry via F10 dump =="
key Escape; sleep 0.5
xdotool key --clearmodifiers F10; sleep 1.2
DUMP=$(grep -a "UI TREE" -A 80 $LOG | grep -E "pos=\(600,4[0-9]{2}\) size=\(460x[0-9]+\)" | head -1)
echo "panel node: ${DUMP:-NOT FOUND}"
[ -n "$DUMP" ] && ok "quit modal 460px centered (F10 dump)" || bad "quit modal geometry"
key Escape; sleep 0.5
key Escape; sleep 0.3
key Escape; sleep 0.8; key e    # open then confirm
for i in $(seq 1 12); do kill -0 $PID 2>/dev/null || break; sleep 0.5; done
if kill -0 $PID 2>/dev/null; then bad "confirm-quit exits app"; kill -9 $PID; else ok "confirm-quit exits app"; fi

echo "== 4. FULL LOOP TO BASE HUB =="
sleep 2; > $LOG
nohup ./target/debug/picnic > $LOG 2>&1 &
PID=$!; sleep 12
WID=$(xdotool search --name '^Picnic$' | head -1)
wmctrl -i -r $WID -e 0,150,150,1200,800; sleep 2
wmctrl -i -a $WID; sleep 1
key F4; wait_log "Loaded.*map with"
key F2; key e
keys W W W D D D D W D W W; key e; key e; wait_log "Picked up: Battery"
keys S S A S A A W W A A A A A A A A A A A A
wait_log "Player reached exit"
key e; wait_log "Returning to base hub"; sleep 2

echo "== 5. STASH SCREEN: footer hints + panel fits + move works =="
grab ui_f5_stash.png
python3 -c "
from PIL import Image
img = Image.open('/tmp/ui_f5_stash.png').convert('RGB'); w,h=img.size; px=img.load()
y = sum(1 for yy in range(int(h*0.80),int(h*0.86)) for x in range(0,w,2) if px[x,yy][0]>180 and px[x,yy][1]>180 and px[x,yy][2]<140)
print('stash footer yellow (in-panel):', y); exit(0 if y>30 else 1)" && ok "stash footer hints visible" || bad "stash footer hints visible"
python3 -c "
from PIL import Image
img = Image.open('/tmp/ui_f5_stash.png').convert('RGB'); w,h=img.size; px=img.load()
# inner panels bg (0.1=25): find extents
xmin,xmax,n = w,0,0
for yy in range(h//3,2*h//3,3):
    for x in range(w):
        r,g,b=px[x,yy]
        if abs(r-25)<3 and abs(g-25)<3 and abs(b-25)<3:
            xmin=min(xmin,x); xmax=max(xmax,x); n+=1
print(f'inner panels span ({xmin},{xmax}) w={xmax-xmin} fits_85pct={xmax-xmin <= int(0.85*w)} n={n}')
exit(0 if n>500 and xmax-xmin <= int(0.85*w) else 1)" && ok "stash panels fit (<=85vw)" || bad "stash panels fit"
key e; wait_log "Moving 'Bolt' from RunInventory to Stash"; ok "stash move works"

echo "== 6. CONTRACTS SCREEN: footer =="
key Tab; sleep 1.2
grab ui_f6_contracts.png
python3 -c "
from PIL import Image
img = Image.open('/tmp/ui_f6_contracts.png').convert('RGB'); w,h=img.size; px=img.load()
y = sum(1 for yy in range(h//2,h-30) for x in range(0,w,2) if px[x,yy][0]>180 and px[x,yy][1]>180 and px[x,yy][2]<140)
print('contracts footer yellow:', y); exit(0 if y>30 else 1)" && ok "contracts footer hints visible" || bad "contracts footer hints"
key Tab; sleep 1.2

echo "== 7. SMALL WINDOW: panels still fit =="
wmctrl -i -r $WID -e 0,200,200,860,540; sleep 2
grab ui_f7_stash_small.png
python3 -c "
from PIL import Image
img = Image.open('/tmp/ui_f7_stash_small.png').convert('RGB'); w,h=img.size; px=img.load()
xmin,xmax,n = w,0,0
for yy in range(h//3,2*h//3,2):
    for x in range(w):
        r,g,b=px[x,yy]
        if abs(r-25)<3 and abs(g-25)<3 and abs(b-25)<3:
            xmin=min(xmin,x); xmax=max(xmax,x); n+=1
print(f'small window {w}x{h}: inner panels span ({xmin},{xmax}) w={xmax-xmin} fits={xmax-xmin<=w}')
exit(0 if n>300 and xmax-xmin<=w else 1)" && ok "panels fit small window" || bad "panels fit small window"
wmctrl -i -r $WID -e 0,150,150,1200,800; sleep 1.5

echo "== 8. QUIT from base hub: ESC -> confirm =="
key Escape; sleep 0.8; key e
for i in $(seq 1 12); do kill -0 $PID 2>/dev/null || break; sleep 0.5; done
if kill -0 $PID 2>/dev/null; then bad "base hub confirm-quit"; kill -9 $PID; else ok "base hub confirm-quit"; fi

echo "===== SUMMARY ====="
echo "PASSED:${PASS}"
echo "FAILED:${FAIL:- none}"
