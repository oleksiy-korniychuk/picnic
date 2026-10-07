#!/usr/bin/env python3
"""Capture a window's CLIENT area exactly (decorations excluded).

Usage: grab_window.py <window_id> <output.png>

Uses xwininfo's absolute client geometry, which already excludes the frame
(xdotool getwindowgeometry reports the frame origin, offsetting Y by the
title-bar height - the source of a subtle screenshot-offset bug).
"""
import subprocess, sys
from PIL import ImageGrab

wid = sys.argv[1]
out = sys.argv[2]
info = subprocess.run(["xwininfo", "-id", wid], capture_output=True, text=True).stdout
def field(name):
    for line in info.splitlines():
        if name in line:
            return int(line.split()[-1])
    raise SystemExit(f"missing {name}")
x, y, w, h = field("Absolute upper-left X:"), field("Absolute upper-left Y:"), field("Width:"), field("Height:")
img = ImageGrab.grab(xdisplay=":0").convert("RGB")
img.crop((x, y, x + w, y + h)).save(out)
print(f"grabbed {w}x{h} at ({x},{y}) -> {out}")
