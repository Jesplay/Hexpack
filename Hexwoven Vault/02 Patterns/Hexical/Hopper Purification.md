---
aliases: ["Hopper Purification"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:index_hopper"
signature: "qqqeawqwawq"
start_direction: "SOUTH_WEST"
input: "source"
output: "list"
cost: ""
iotas_in: []
iotas_out: ["[[List]]"]
entry: "[[Hopper (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hopper Purification

◇ **Pattern** · [[Hexical]] · 📖 [[Hopper (Book Entry)]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 65.7,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 65.7,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="40.2" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,78.9 36.5,69.1 43.9,73.3" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqeawqwawq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `source` (—)
- **Leaves:** `list` ([[List]])
- **ID:** `hexical:index_hopper`

```text
before   … │ source  ◂ top
after    … │ list    ◂ top
```

## Description

Applies to **sources** that support slots; inventories. Returns a list of the item stacks contained within them, with their index in the list being their slot number.

While the slot number is generally fairly logical, it can feel arbitrary for storages such as a player's inventory.

This pattern will be useful in ascertaining the slot number of any given location in a **source** or **destination**'s inventory. It can also be useful for processing: scanning a **source** for what items it contains, evaluating them based on some metrics, and from there deciding whether and where to move them.

## Related patterns (same book page)

- [[Hopper]] `source, num?, dest., num? → —`

## Sources

- 📖 **Guidebook:** [[Hopper (Book Entry)]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpIndexHopper` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:index_hopper`
