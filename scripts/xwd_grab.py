#!/usr/bin/env python3
"""Capture a window's content directly via xwd (decorations excluded,
position-independent). Correct XWDFileHeader parsing.

Usage: xwd_grab.py <window_id> <output.png>
"""
import struct, subprocess, sys
from PIL import Image

wid, out = sys.argv[1], sys.argv[2]
data = subprocess.run(["xwd", "-id", wid, "-silent"], capture_output=True).stdout
if not data or len(data) < 100:
    raise SystemExit("xwd failed")

# XWDFileHeader - all big-endian u32:
hdr = struct.unpack(">25I", data[:100])
(header_size, version, fmt, depth, width, height, xoffset,
 byte_order, bitmap_unit, bitmap_bit_order, bitmap_pad,
 bits_per_pixel, bytes_per_line, visual_class,
 red_mask, green_mask, blue_mask, bits_per_rgb, colormap_entries) = (
    hdr[0], hdr[1], hdr[2], hdr[3], hdr[4], hdr[5], hdr[6],
    hdr[7], hdr[8], hdr[9], hdr[10],
    hdr[11], hdr[12], hdr[13],
    hdr[14], hdr[15], hdr[16], hdr[17], hdr[19])

if version != 7:
    raise SystemExit(f"unexpected xwd version {version}")

pixdata = data[header_size + colormap_entries * 12:]

bpp = bits_per_pixel // 8
pix = bytearray(height * width * 3)
for y in range(height):
    row = pixdata[y * bytes_per_line : (y + 1) * bytes_per_line]
    o = y * width * 3
    for x in range(width):
        if bpp == 4:
            b0, b1, b2 = row[x*4], row[x*4+1], row[x*4+2]
            if byte_order == 0:   # LSBFirst: B G R X
                r, g, bl = b2, b1, b0
            else:                 # MSBFirst: X R G B
                r, g, bl = b1, b2, b3 if x*4+3 < len(row) else b2
        else:                     # bpp == 3: B G R
            b0, b1, b2 = row[x*3], row[x*3+1], row[x*3+2]
            r, g, bl = b2, b1, b0
        pix[o] = r; pix[o+1] = g; pix[o+2] = bl; o += 3

img = Image.frombytes("RGB", (width, height), bytes(pix))
img.save(out)
print(f"grabbed {width}x{height} depth={depth} bpp={bits_per_pixel} -> {out}")
