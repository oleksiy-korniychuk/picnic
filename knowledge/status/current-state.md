---
type: Status Snapshot
title: Current state (left-off point)
description: Point-in-time snapshot of the implementation at post-fc71ea6 working tree - bugs 1 and 2 fixed and verified; what is done, what remains.
tags: [picnic, status, snapshot]
status: stable
stale_after: 2026-10-13T00:00:00Z
generated: { by: pi/openai/qwen3.8-27b, at: 2026-10-06T06:00:00Z }
sources:
  - id: commit-fc71ea6
    resource: https://github.com/oleksiy-korniychuk/picnic/commit/fc71ea6
    title: Commit fc71ea6 - stash system + base hub item management ("might be untested" per commit message)
    author: human:oleksiy
    last_modified: 2026-07-11T23:15:14Z
  - id: bugfix-commits
    resource: https://github.com/oleksiy-korniychuk/picnic/commit/8235845
    title: Commits 8235845 (extraction wipe fix) + 9e3995a (rebuild/double-spawn fix) - the bug fixes below, committed and pushed
    author: agent:pi/openai/qwen3.8-27b
    last_modified: 2026-10-07T04:45:00Z
  - id: roadmap
    resource: /roadmap/v0.1-remaining.md
    title: v0.1 remaining work
    author: human:oleksiy
  - id: progression
    resource: /design/progression.md
    title: Progression (v0.1)
    author: human:oleksiy
---

# Implemented

Everything from [commit fc71ea6](https://github.com/oleksiy-korniychuk/picnic/commit/fc71ea6)[^commit-fc71ea6] (stash resources, data-driven stash screen, zone/base item handoff, permadeath wiring) plus the two bug fixes below (committed as 8235845 and 9e3995a).[^bugfix-commits]

## Meta loop verified as working (2026-10-07)

The full repeatable loop **zone → loot → extract → base hub (stash) → re-enter** already works end-to-end with no new code needed - verified live with a scripted full-loop run (`/tmp/meta_loop.sh`):

* Run 1: pickup Battery, extract → `Saved 12 items ... (weight: 63)`, state preserved.
* Base hub: move a Bolt to stash (stash = 1).
* Re-enter via Space in the Stash Management screen: `Player spawned with 11 items ... (weight: 62)` - exactly 12 minus the stashed Bolt, proving RunInventory/stash persistence across runs.
* Run 2: the Battery tile is still empty (no loot respawn - map persists), Rust Slag pickup works, second extraction → `Saved 12 items ... (weight: 67)`.
* Death in run 3 (gravitational anomaly) → `PERMADEATH: Reset stash and RunInventory to starter loadout` → auto-restart with a fresh starter loadout; app restart is also a full reset (state is in-memory only).

One latent bug fixed while verifying: `Stash` derived `Default` gave `capacity: 0` instead of the designed 1000 (`init_resource` never calls `Stash::new()`); now a manual `Default` impl carries 1000, with unit tests (`cargo test`, 3 passing).

## Bug fixes (2026-10-06)

1. **Extraction no longer wipes the stash and run inventory** (was bug 1, live-confirmed then fixed). `AutoRestartFlag` gained a `permadeath` marker; `close_death_ui_system` sets it when the player confirms the death screen, and `prepare_restart_system` (`OnExit(GameState::Running)`, `src/systems/contract_ui.rs`) now consumes the flag and only performs the full wipe (stash clear, `RunInventory` starter reset, contract/turn/log reset) when it is set. Extraction and the F2 editor toggle exit `Running` without the marker and log `Leaving the Zone - stash, RunInventory and contracts preserved` instead. Verified live (see [Verification](#verification)).
2. **Stash UI rebuild path completed** (was bug 2). The "UI Rebuilding..." stub `spawn_stash_management_ui_with_selection` is gone; a shared `spawn_stash_ui_with_selection` builder in `src/systems/base_hub_ui.rs` now backs both the initial spawn and the data-change rebuild, so a rebuilt screen is pixel-identical to a fresh one. Also fixed in the same area:
   * `handle_stash_ui_spawn_system` now despawns the stash screen when Tab switches away from StashManagement mode (it previously did nothing, leaving the stash UI stacked under the Contracts screen).
   * The base hub Update systems are `.chain()`ed in `main.rs` (spawn handlers → navigation → move → highlighting → rebuild → mode toggle/enter/exit). Before, `handle_stash_ui_spawn_system` and `rebuild_stash_ui_system` were unordered, could both defer a spawn of the screen root, and end up with two stacked UI roots - live-confirmed pre-fix (double-dark overlay, `E` move silently no-oped on `get_single_mut` failure). `rebuild_stash_ui_system` additionally bails when no root exists.

# Known bugs

None currently open. (The two bugs above were reproduced, fixed, and re-verified the same day.)

# Not yet implemented

Per the [roadmap](/roadmap/v0.1-remaining.md) and [progression design](/design/progression.md):[^progression]

* **Money system** - no `RunMoney` resource exists; the base hub shows a hardcoded "Money: 0 Rubles".
* **Selling items** - S-to-sell from either base hub inventory is not wired; item values exist on `Item` but are display-only.
* **Contract system** - `ContractSystem` still holds one hardcoded "Fully Empty" contract; the base hub Contracts screen is fully placeholder (5 fake available, 0/3 active). No pool generation (5 random), no reward field, no activation (max 3), no turn-in flow.
* **Base hub message log** - no messages for moves/sells/contract completions.
* Pre-existing cosmetic issue: `Items.png` fails to load (asset path resolves to `target/debug/assets/`, not the repo `assets/`), so item sprites error-spam the log every frame after map load. Ground-item tile colors still render; low priority.

# Verification

* **Compile-verified** (2026-10-06, post-fix): `cargo check` + `cargo build` pass; 43 warnings (pre-existing dead code), 0 errors.
* **Runtime-verified** (2026-10-06, post-fix) with a scripted harness (xdotool key injection, per-step process/focus guards, log polling, PIL pixel checks; window geometry read dynamically - see the environment note about the position footgun):
  * **Extraction preserves state (bug 1 fix)**: picked up a Battery in the Zone (log: `Picked up: Battery (weight: 3)`), extracted at the exit (`Saved 12 items from player to RunInventory (weight: 63)`), and the log shows **no** `PERMADEATH` line - instead `Leaving the Zone - stash, RunInventory and contracts preserved`. Pre-fix, the same flow logged the wipe 17 ms after the save.
  * **Base hub rebuild works (bug 2 fix)**: two E item-moves both completed (`Moving 'Bolt' from RunInventory to Stash` x2); screenshots after each move show the real two-panel UI (~4000 unique colors), not the old single-line stub. Tab to Contracts and back renders a clean stash screen; the after-move and after-tab-back screenshots are pixel-identical.
  * **F2 is a real editor toggle now**: F2 out of the Zone → editor (1 spawn, 0 permadeath), F2 back → respawn (2 spawns, 0 permadeath). Pre-fix this bounced straight back into a fresh run.
  * **Death still triggers permadeath**: scripted walk into the gravitational anomaly at (5,11) (adjacent-tile pull → 5-turn timer → bounce W/S in range). Log: `DEATH: Player was crushed by gravitational anomaly!` → on death-screen E: `PERMADEATH: Reset stash and RunInventory to starter loadout` → `Auto-restarting game` → `Player spawned with 11 items ... weight: 60` (starter loadout).
  * Graceful exit from base hub via ESC (`Quitting game from base hub`), no panics anywhere.
  * **Full meta loop (2026-10-07)**: three consecutive runs in one session with stash deposits between them - spawn weights 60 → 62 → 66 match RunInventory minus stash plus pickups; Battery picked exactly once across all runs (no respawn); death reset and relaunch reset both leave a clean starter state. (Details under [Implemented](#implemented).)
* **Environment**: rustup stable at `~/.cargo/bin`; Nix not installed on this machine (native toolchain path); `xdotool` + Pillow available for scripted validation; see [AGENTS.md](../../../AGENTS.md) for the package-install rule and machine notes. Harness footguns learned this session: query window geometry per run (the WM opened the window at (60,122) instead of (50,82) once); picking up the *last* item on a tile auto-closes the inspect UI, so a scripted ESC afterwards would hit the global quit handler.

# Next steps

In suggested order:

1. Money system (`RunMoney`) + selling items (roadmap items 2 and 3).
3. Contract system expansion (roadmap item 4).
4. Base hub polish: message log, data-driven contract screen, remove placeholders (roadmap item 6).
5. Optionally relocate the validation harness scripts from `/tmp` (`baseline.sh`, `val_a.sh`, `val_bc.sh`) into `scripts/` as regression tests.

# Bundle hygiene

At snapshot time the working tree (bug fixes + bundle) was uncommitted; it was committed and pushed shortly after this snapshot was written. The knowledge bundle is tracked in git from that point on - future bundle edits should be committed alongside the work that caused them.

[^commit-fc71ea6]: Commit fc71ea6 - stash system + base hub item management ("might be untested" per commit message)
[^bugfix-working-tree]: Local uncommitted changes applying the two bug fixes
[^progression]: Progression (v0.1)
