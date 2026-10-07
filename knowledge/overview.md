---
type: Project Overview
title: Picnic
description: A turn-based roguelike inspired by Roadside Picnic, built with Rust and Bevy, currently in early development.
tags: [picnic, roguelike, bevy, rust]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: readme
    resource: /references/source-docs/README.md
    title: Picnic README
    author: human:oleksiy
    last_modified: 2025-11-23T22:04:19Z
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
---

# Summary

Picnic is a roguelike game inspired by *Roadside Picnic*, built with Rust and the Bevy game engine (v0.16.0) with its ECS framework. The project is currently in early development - cleaning up infrastructure and preparing for core gameplay implementation.[^readme]

# Proof of concept

The POC (see the [POC design document](/references/source-docs/POC.md)) is a turn-based roguelike where players explore a 25x25 Zone, detect anomalies using bolts, collect items and artifacts, and extract safely.[^poc]

* 25x25 tile map, handcrafted via the in-game [tilemap editor](/systems/tilemap-editor.md).
* Turn-based [core loop](/design/core-loop.md) with contract-based [runs](/design/contracts.md).
* Three anomaly types: Gravitational Anomaly, Philosopher's Stone, The Rust (see [anomalies](/design/anomalies.md)).
* Anomaly detection by [throwing bolts](/systems/bolt-throwing.md); the [metal detector](/systems/inventory.md) finds metal items.
* 4-directional movement only; traditional roguelike controls and UI patterns; no combat, no audio (POC scope).[^poc]
* [UI standards](/systems/ui-standards.md) shared by all modal screens.

[^readme]: Picnic README
[^poc]: POC Design Document
