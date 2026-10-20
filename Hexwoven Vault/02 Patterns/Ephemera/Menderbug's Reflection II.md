---
aliases: ["Menderbug's Reflection II"]
tags: [hexcasting/pattern, mod/ephemera, family/reflection, topic/items]
kind: "Pattern"
family: "Reflection"
topics: ["Items & Crafting"]
mod: "Ephemera"
op_id: "ephemera:getdurabilityoffhand"
signature: "aedwq"
start_direction: "EAST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Induce Mending (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Menderbug's Reflection II

◇ **Pattern** · [[Ephemera]] · 📖 [[Induce Mending (Book Entry)]] · Patterns › Great Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,94.0 65.7,94.0 46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 143.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:232;--dur:1.7s" points="27.0,94.0 65.7,94.0 46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 143.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="94.0" x2="39.4" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,94.0 39.4,98.2 39.4,89.7" fill="#fc77be"/><circle cx="27.0" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aedwq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `ephemera:getdurabilityoffhand`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

As previous, targeting the item in the hand I'm casting this with. Most useful with delayed casts of one form or another.

## Related patterns (same book page)

- [[Induce Mending]] `entity → —`
- [[Menderbug's Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Induce Mending (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Menderbug's Reflection II](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/great/mending@ephemera:getdurabilityoffhand)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpHandDurability` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getdurabilityoffhand`
