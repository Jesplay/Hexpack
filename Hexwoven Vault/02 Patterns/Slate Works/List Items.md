---
aliases: ["List Items"]
tags: [hexcasting/pattern, mod/slate-works, family/other, topic/circles]
kind: "Pattern"
family: "Other"
topics: ["Spell Circles, Loci & Engines"]
mod: "Slate Works"
op_id: "slate_work:get_items"
signature: "eaqwqaeqawawaedd"
start_direction: "SOUTH_WEST"
input: ""
output: "[item stack]"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Item Stack]]"]
entry: "[[Pocket Simulator Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# List Items

◇ **Pattern** · [[Slate Works]] · 📖 [[Pocket Simulator Patterns]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="85.0,60.5 65.7,94.0 27.0,94.0 46.4,127.4 85.0,127.4 123.6,127.4 143.0,94.0 104.3,94.0 85.0,60.5 46.4,60.5 65.7,94.0 85.0,127.4 104.3,94.0 123.6,60.5 85.0,60.5 65.7,27.0 104.3,27.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:657;--dur:4.8s" points="85.0,60.5 65.7,94.0 27.0,94.0 46.4,127.4 85.0,127.4 123.6,127.4 143.0,94.0 104.3,94.0 85.0,60.5 46.4,60.5 65.7,94.0 85.0,127.4 104.3,94.0 123.6,60.5 85.0,60.5 65.7,27.0 104.3,27.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="60.5" x2="78.8" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,78.9 75.1,69.1 82.5,73.3" fill="#fc77be"/><circle cx="85.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaeqawawaedd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[item stack]` ([[List]], [[Item Stack]])
- **ID:** `slate_work:get_items`

```text
before   …                 ◂ top
after    … │ [item stack]  ◂ top
```

## Description

This returns all the items held within the [[Pocket Simulator]] as [[Items|Item Stack Iotas]]. Free.

## Related patterns (same book page)

- [[Set Slot]] `int → —`

## Sources

- 📖 **Guidebook:** [[Pocket Simulator Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetItems` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:get_items`
