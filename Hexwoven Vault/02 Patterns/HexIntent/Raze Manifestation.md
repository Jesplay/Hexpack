---
aliases: ["Raze Manifestation"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:destroy_manifestation"
signature: "edeeedwwaq"
start_direction: "NORTH_WEST"
input: "vec, num"
output: "none"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Manifest Threshold (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Raze Manifestation

◇ **Pattern** · [[HexIntent]] · 📖 [[Manifest Threshold (Book Entry)]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,94.0 65.7,60.5 85.0,27.0 104.3,60.5 85.0,94.0 46.4,94.0 27.0,60.5 65.7,60.5 104.3,60.5 143.0,60.5 123.6,27.0 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="85.0,94.0 65.7,60.5 85.0,27.0 104.3,60.5 85.0,94.0 46.4,94.0 27.0,60.5 65.7,60.5 104.3,60.5 143.0,60.5 123.6,27.0 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="94.0" x2="78.8" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,75.6 82.5,81.1 75.1,85.4" fill="#fc77be"/><circle cx="85.0" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeeedwwaq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `none` (—)
- **ID:** `manifestation:destroy_manifestation`

```text
before   … │ vec │ num  ◂ top
after    … │ none       ◂ top
```

## Description

Takes an origin vector and a radius, destroys manifestations (portals and relays) in that area. Based on permissions.

## Other uses (same pattern, other inputs)

### [[HexIntent]] · 📖 [[Manifest Relay (Book Entry)]]

`vec, num` → `none`

Takes an origin vector and a radius, destroys manifestations (portals and relays) in that area. Based on permissions.

## Related patterns (same book page)

- [[Manifest Threshold]] `vec, presence, num, text?, pigment? → none`
- [[Presence Intent]] `vec, vec → presenceIntent`

## Sources

- 📖 **Guidebook:** [[Manifest Threshold (Book Entry)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDestroyManifestation` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:destroy_manifestation`
