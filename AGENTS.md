# AGENTS.md

Notes for coding agents (and humans) working on this machine.

## Installing system packages

**ALWAYS ask the user before installing any new package.** `sudo` requires a
password, so agents cannot install packages non-interactively. When a build or
runtime step is blocked by a missing package:

1. Identify the exact package name(s).
2. Ask the user to run the install command themselves, e.g.
   `sudo apt-get install -y libasound2-dev libudev-dev`
3. Do not patch project files (Cargo.toml, flake.nix, ...) to work around a
   missing system package without asking first.

(Requested by the user, 2026-10-06.)

## Environment

- Linux Mint 22.3 (Ubuntu 24.04 base), X11 session (`DISPLAY=:0`).
- Rust via rustup at `~/.cargo/bin` (add to `PATH`); stable toolchain, minimal profile.
- Nix is **not** installed on this machine; `flake.nix` exists but is currently
  unused here. Native toolchain (rustup + apt) is the working path.
- Build deps already installed: `build-essential`, `pkg-config`,
  `libasound2-dev`, `libudev-dev`. Bevy runtime libs (X11, xkbcommon, wayland,
  alsa, udev) are present system-wide.
- Vulkan works: Intel UHD 630 (Mesa), NVIDIA GTX 1050 Ti Max-Q, and llvmpipe
  (software) fallback. Hybrid graphics — if the game picks the wrong GPU, set
  `VK_ICD_FILENAMES` or run with `WGPU_BACKEND=gl`.
