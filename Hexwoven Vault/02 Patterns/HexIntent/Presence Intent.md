---
aliases: ["Presence Intent"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:presence_intent"
signature: "edewqaqdeeeweee"
start_direction: "NORTH_WEST"
input: "vec, vec"
output: "presenceIntent"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Manifest Threshold (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Presence Intent

◇ **Pattern** · [[HexIntent]] · 📖 [[Manifest Threshold (Book Entry)]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 76.9,85.0 111.8,85.0 129.3,115.3 111.8,145.5 76.9,145.5 41.9,145.5 24.5,115.3 41.9,85.0 76.9,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:559;--dur:4.5s" points="76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 76.9,85.0 111.8,85.0 129.3,115.3 111.8,145.5 76.9,145.5 41.9,145.5 24.5,115.3 41.9,85.0 76.9,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="85.0" x2="71.3" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,68.4 74.6,73.4 68.0,77.2" fill="#fc77be"/><circle cx="76.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edewqaqdeeeweee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `presenceIntent` (—)
- **ID:** `manifestation:presence_intent`

```text
before   … │ vec │ vec       ◂ top
after    … │ presenceIntent  ◂ top
```

## Description

Takes a location and a directional vector and creates an intent. Used with manifest threshold.
Cost: 35 amethyst.

## Related patterns (same book page)

- [[Manifest Threshold]] `vec, presence, num, text?, pigment? → none`
- [[Raze Manifestation]] `vec, num → none`

## Sources

- 📖 **Guidebook:** [[Manifest Threshold (Book Entry)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPresenceIntent` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:presence_intent`
