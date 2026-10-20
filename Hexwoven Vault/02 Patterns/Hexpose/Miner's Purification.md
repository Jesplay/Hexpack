---
aliases: ["Miner's Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:block_hardness"
signature: "qaqqqqqeeeeedq"
start_direction: "EAST"
input: "identifier"
output: "number"
cost: ""
iotas_in: ["[[Identifier]]"]
iotas_out: ["[[Number]]"]
entry: "[[Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Miner's Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Blocks]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="24.3,84.4 59.0,84.4 76.3,54.3 41.6,54.3 24.3,84.4 41.6,114.4 76.3,114.4 93.7,84.4 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 111.0,54.3 93.7,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:520;--dur:4.2s" points="24.3,84.4 59.0,84.4 76.3,54.3 41.6,54.3 24.3,84.4 41.6,114.4 76.3,114.4 93.7,84.4 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 111.0,54.3 93.7,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="84.4" x2="35.4" y2="84.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,84.4 35.4,88.2 35.4,80.6" fill="#fc77be"/><circle cx="24.3" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqqeeeeedq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `identifier` ([[Identifier]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:block_hardness`

```text
before   … │ identifier  ◂ top
after    … │ number      ◂ top
```

## Description

Pushes how difficult blocks of this type are to mine.

## Related patterns (same book page)

- [[Demoman's Purification]] `identifier → number`
- [[Orientation Purification]] `vector → vector/null`
- [[Farmer's Purification]] `vector → number/null`
- [[Facet Purification]] `vector → list of identifiers`
- [[Facet Distillation]] `vector, identifier → any`
- [[Cartographer's Purification]] `identifier → vector`

## Sources

- 📖 **Guidebook:** [[Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Miner's Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/blocks@hexpose:block_hardness)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetBlockTypeData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:block_hardness`
