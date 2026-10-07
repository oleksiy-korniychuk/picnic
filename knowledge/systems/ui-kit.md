---
type: System Spec
title: UI kit (key bar + modal scaffolding)
description: The Zellij-inspired keyboard-first navigation layer - context key bar, shared modal builder, scrolling, and quit confirmation. Lives in src/systems/ui_kit.rs.
tags: [picnic, systems, ui, live]
status: stable
generated: { by: pi/openai/qwen3.8-27b, at: 2026-10-07T16:40:00Z }
sources:
  - id: ui-standards
    resource: /systems/ui-standards.md
    title: UI standards (typography, palette, modal styling)
    author: human:oleksiy
---

# Two-layer navigation (Zellij-inspired)

Navigation is keyboard-first with two layers, used only where each earns its keep:[^user]

* **Layer 1 - the key bar** ([`spawn_key_bar`]): a full-width bottom strip (`28px`, `src/systems/ui_kit.rs`) showing `[key] action` pairs for the current context. The key is highlighted yellow, the action dim gray, always adjacent.
* **Layer 2 - full-screen menus**: the game's modals and base hub screens themselves. Each advertises its own keys in a footer hint row using the same `[key] action` format ([`spawn_hint_row`]). No separate help overlay exists - contexts today have few enough bindings that Layer 1 covers them; add one only if a context grows too many actions.[^user]

## Key bar contexts

[`maintain_key_bar_system`] runs every frame and keeps exactly one bar alive, matching the current context (self-healing, no system-order dependencies):

| Context | Hints |
|---------|-------|
| Editing | `[F2] Play [Tab] Mode [1-6] Tool [LMB] Place [Wheel] Zoom [F3] Save [F4] Load [Esc] Quit` |
| Running / PlayerTurn | `[WASD] Move [E] Inspect [Tab] Bag [Q] Bolt [F2] Editor [Esc] Quit` |
| InBaseHub / StashManagement | `[W/S] Select [A/D] Panel [E] Move [Tab] Contracts [Space] Enter Zone [Esc] Quit` |
| InBaseHub / Contracts | `[Tab] Stash [Esc] Quit` |
| any modal phase | no bar - the modal's footer takes over (Layer 2 replaces Layer 1 while open) |

While a modal is open its overlay (z 100) covers the bar (z 90).

# Modal scaffolding

Every modal is built with [`spawn_modal`] (or [`spawn_modal_ex`] for accent styling - the death screen's red border and darker overlay):

* Full-screen overlay `rgba(0,0,0,0.8)`, z 100, centered panel `rgb(0.15,0.15,0.15)` with gray border.
* **Responsive sizing**: `width: Px(ideal)` (definite, so percent children resolve), `min_width` floor, `max_width: Vw(...)` cap, `max_height: Vh(88)`. Panels shrink on small windows; verify via window resize.
* Title (24px yellow), separator, content area (optionally `overflow: scroll_y` + [`ModalScrollArea`]), pinned footer: separator + hint row.
* Current panel sizes: briefing/extraction/death 520/380px, inspect 540/400, inventory 640/480, stash 900/640 (85vw cap), contracts 700/560 (80vw cap), quit-confirm 460/360 (60vw cap).

## Scrolling

* bevy_ui does not scroll overflow nodes by itself; [`modal_wheel_scroll_system`] wheel-scrolls any [`ModalScrollArea`] under the cursor (hit-testing mirrors bevy_ui's picking backend). The wheel must not also zoom the camera: [`zoom_allowed`] gates `camera_zoom_system` off in the base hub, modal phases, and while quit-confirm is open.
* Keyboard selection auto-scrolls via [`scroll_selection_into_view`] (inspect, inventory, base hub panels) using measured `ComputedNode` sizes - no hardcoded row heights.

## Base hub screens

The stash and contracts screens are full-screen overlays built on the same scaffolding (they carry `ModalRoot` but must NOT gate input via `no_modal_open` - they are the active screen). Each panel list is its own `ModalScrollArea`; the footer repeats the bar hints because the overlay covers the bottom bar.

# Quit confirmation (ESC hardening)

ESC no longer quits instantly. [`escape_menu_system`] opens a confirm modal in top-level contexts (Editing, Running/PlayerTurn, InBaseHub); [`quit_confirm_resolve_system`]: `E` exits, `Esc` stays. A `just_opened` flag swallows the same keypress that opened it. Modal-phase ESC keys are unchanged (inspect/inventory close to WorldUpdate; briefing skips; extraction cancels; death requires E). Input-sensitive system groups are gated with [`no_modal_open`] - which keys on `QuitConfirmUiRoot`, not on all modals.

# Diagnostics

[`debug_ui_dump_system`] (F10) prints the whole UI tree - entity, computed position (center, logical px), computed size and content size. Use it as the geometric oracle when validating layout: pixel-color probing is unreliable because Bevy blends UI in linear space over an sRGB framebuffer, so overlay-over-floor (gray 84) measures ~37 - nearly identical to the panel color (38) - and scene bands alias with panel runs.

# Validation

Scripted UI validation harness (xdotool key injection + window-relative `xwd` captures + F10 dumps + log polling): `scripts/ui_final.sh`, `scripts/meta_loop.sh`, `scripts/xwd_grab.py`. Captures must be window-relative (xwd) - the window moves/resizes on screen and `xdotool getwindowgeometry` reports the frame origin, offset by the title bar.

[^user]: Design direction from the 2026-10-07 UI/UX overhaul discussion: balance between lightweight context hints and full-screen menus, keybinds always shown next to the short action name, prefer traditional full-screen menus where simpler.
