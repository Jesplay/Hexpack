---
aliases: ["Orientation Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:blockstate_rotation"
signature: "qaqqqqqwadeeed"
start_direction: "EAST"
input: "vector"
output: "vector/null"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]", "[[Null]]"]
entry: "[[Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Orientation Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Blocks]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,115.3 59.4,115.3 76.9,85.0 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 94.4,115.3 76.9,85.0 59.4,54.7 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 59.4,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:524;--dur:4.2s" points="24.5,115.3 59.4,115.3 76.9,85.0 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 94.4,115.3 76.9,85.0 59.4,54.7 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 59.4,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="115.3" x2="35.6" y2="115.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.7,115.3 35.6,119.1 35.6,111.4" fill="#fc77be"/><circle cx="24.5" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqqwadeeed` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `vector/null` ([[Vector]], [[Null]])
- **ID:** `hexpose:blockstate_rotation`

```text
before   … │ vector       ◂ top
after    … │ vector/null  ◂ top
```

## Description

Pushes the vector of the block's facing direction, or Null if blocks of its type do not have an orientation.

## Related patterns (same book page)

- [[Miner's Purification]] `identifier → number`
- [[Demoman's Purification]] `identifier → number`
- [[Farmer's Purification]] `vector → number/null`
- [[Facet Purification]] `vector → list of identifiers`
- [[Facet Distillation]] `vector, identifier → any`
- [[Cartographer's Purification]] `identifier → vector`

## Sources

- 📖 **Guidebook:** [[Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Orientation Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/blocks@hexpose:blockstate_rotation)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetBlockStateData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:blockstate_rotation`
