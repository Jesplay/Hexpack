---
aliases: ["Cartographer's Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:block_map_color"
signature: "qwedewqqqqq"
start_direction: "EAST"
input: "identifier"
output: "vector"
cost: ""
iotas_in: ["[[Identifier]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cartographer's Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Blocks]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,94.0 65.7,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 85.0,60.5 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="27.0,94.0 65.7,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 85.0,60.5 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="94.0" x2="39.4" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,94.0 39.4,98.2 39.4,89.7" fill="#fc77be"/><circle cx="27.0" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwedewqqqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `identifier` ([[Identifier]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexpose:block_map_color`

```text
before   … │ identifier  ◂ top
after    … │ vector      ◂ top
```

## Description

Pushes the color blocks of this type appear as on a map.

## Related patterns (same book page)

- [[Miner's Purification]] `identifier → number`
- [[Demoman's Purification]] `identifier → number`
- [[Orientation Purification]] `vector → vector/null`
- [[Farmer's Purification]] `vector → number/null`
- [[Facet Purification]] `vector → list of identifiers`
- [[Facet Distillation]] `vector, identifier → any`

## Sources

- 📖 **Guidebook:** [[Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Cartographer's Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/blocks@hexpose:block_map_color)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetBlockTypeData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:block_map_color`
