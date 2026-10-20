---
aliases: ["Destroy Splinters"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:destroy_splinters"
signature: "dedadeaqaww"
start_direction: "SOUTH_WEST"
input: ""
output: "none"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Splinters]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Destroy Splinters

◇ **Pattern** · [[HexIntent]] · 📖 [[Splinters]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="59.5" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,78.9 55.8,69.1 63.2,73.3" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedadeaqaww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `none` (—)
- **ID:** `manifestation:destroy_splinters`

```text
before   …         ◂ top
after    … │ none  ◂ top
```

## Description

Dismisses all splinters you have active.

## Related patterns (same book page)

- [[Manifest Splinter]] `vec, num, list → none`
- [[Renew Splinter]] `vec, num → none`
- [[Saturn's Gambit]] `pattern → none`
- [[Get Splinter Location]] `— → vec`
- [[Destroy Current Splinter]] `— → none`

## Sources

- 📖 **Guidebook:** [[Splinters]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDestroySplinters` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:destroy_splinters`
