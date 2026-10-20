---
aliases: ["Set Slot"]
tags: [hexcasting/pattern, mod/slate-works, family/other, topic/circles]
kind: "Pattern"
family: "Other"
topics: ["Spell Circles, Loci & Engines"]
mod: "Slate Works"
op_id: "slate_work:set_slot"
signature: "eaqwqaeqawawa"
start_direction: "SOUTH_WEST"
input: "int"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Pocket Simulator Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Set Slot

◇ **Pattern** · [[Slate Works]] · 📖 [[Pocket Simulator Patterns]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 85.0,94.0 104.3,60.5 123.6,27.0 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="85.0,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 85.0,94.0 104.3,60.5 123.6,27.0 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="78.8" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,45.4 75.1,35.6 82.5,39.9" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaeqawawa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `int` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `slate_work:set_slot`

```text
before   … │ int  ◂ top
after    …        ◂ top
```

## Description

This takes a number between 0 and 5 (inclusive), and sets the held slot of the bound [[Pocket Simulator]]. Free.

## Related patterns (same book page)

- [[List Items]] `— → [item stack]`

## Sources

- 📖 **Guidebook:** [[Pocket Simulator Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetSlot` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:set_slot`
