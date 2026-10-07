---
type: System Spec
title: UI standards
description: Shared typography, text-label, modal styling, and color palette rules for all modal screens.
tags: [picnic, systems, ui, standards, live]
generated: { by: pi/glm-5.3-flash, at: 2026-10-05T23:23:17Z }
sources:
  - id: poc
    resource: /references/source-docs/POC.md
    title: POC Design Document
    author: human:oleksiy
    last_modified: 2025-12-02T07:35:22Z
  - id: progression
    resource: /references/source-docs/v0.1_progression.md
    title: Progression System Design (v0.1)
    author: human:oleksiy
    last_modified: 2025-12-06T05:40:56Z
---

# Modal screen typography

All modal screens (Mission Briefing, Extraction, Death, Inspect, Inventory) use consistent font sizes:[^poc]

| Element | Font size | Usage |
|---------|-----------|-------|
| Titles | 24.0 | Main screen titles (e.g. "Mission Briefing", "DEATH", "Extraction Point") |
| Subtitles/headers | 16.0 | Section headers (e.g. "Active Contracts:", "Contract Status:") |
| Body text | 18.0 | Item descriptions, contract details, main content |
| Help text | 16.0 | Input prompts (e.g. "E - Accept and Enter the Zone") |
| Status markers | 16.0 | Completion indicators like [COMPLETE]/[FAILED] |

# Text labels

* Use text labels instead of Unicode symbols for better font compatibility.[^poc]
* Status indicators: `[COMPLETE]` (green) and `[FAILED]` (red).[^poc]
* Avoid Unicode characters like ✓ and ✗, which may not render correctly in default fonts.[^poc]

# Modal styling

* Semi-transparent dark overlay (rgba 0,0,0,0.8) behind all modals, z-index 100 for modal overlays.[^poc]
* Modal panels: dark gray background (rgb 0.15,0.15,0.15) with gray borders; minimum width 500px, maximum width 700px; padding 30-40px; row gap 15-20px.[^poc]
* The base hub screens extend this to 700-900px panels with 40px padding and a 20px row gap (see [progression](/design/progression.md)).[^progression]

# Color palette

* Success/positive: green (0.3, 0.9, 0.3) or (0.6, 0.9, 0.6).[^poc]
* Failure/negative: red (0.9, 0.3, 0.3) or (0.9, 0.2, 0.2).[^poc]
* Highlight/important: yellow (0.9, 0.9, 0.3).[^poc]
* Neutral text: white or light gray (0.8, 0.8, 0.8).[^poc]

[^poc]: POC Design Document
[^progression]: Progression System Design (v0.1)
