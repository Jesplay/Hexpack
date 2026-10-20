---
aliases: ["Manifest Threshold"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:open_corridor_portal"
signature: "edqqdeew"
start_direction: "NORTH_WEST"
input: "vec, presence, num, text?, pigment?"
output: "none"
cost: "Cost: 2 dust per second."
iotas_in: ["[[Number]]", "[[Vector]]", "[[String]]", "[[Pigment]]"]
iotas_out: []
entry: "[[Manifest Threshold (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifest Threshold

◇ **Pattern** · [[HexIntent]] · 📖 [[Manifest Threshold (Book Entry)]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="40.2" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,75.6 43.9,81.1 36.5,85.4" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edqqdeew` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, presence, num, text?, pigment?` ([[Number]], [[Vector]], [[String]], [[Pigment]])
- **Leaves:** `none` (—)
- **Cost:** Cost: 2 dust per second.
- **ID:** `manifestation:open_corridor_portal`

```text
before   … │ vec │ presence │ num │ text? │ pigment?  ◂ top
after    … │ none                                     ◂ top
```

## Description

Takes a vector, a Prescence intent, [[Media|media]] allocation and an optional string label and [[Pigments|pigment]]. Opens a portal bridging two locations together.
**Cost: 2 dust per second. 35 Charged Crystal on frame.**

## Related patterns (same book page)

- [[Presence Intent]] `vec, vec → presenceIntent`
- [[Raze Manifestation]] `vec, num → none`

## Sources

- 📖 **Guidebook:** [[Manifest Threshold (Book Entry)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpOpenCorridorPortal` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:open_corridor_portal`
