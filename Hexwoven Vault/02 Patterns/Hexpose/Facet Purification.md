---
aliases: ["Facet Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:get_blockstates"
signature: "qaqqqeqqqwqaww"
start_direction: "EAST"
input: "vector"
output: "list of identifiers"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[List]]", "[[Identifier]]"]
entry: "[[Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Facet Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Blocks]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="24.3,54.3 59.0,54.3 76.3,24.3 41.6,24.3 24.3,54.3 41.6,84.4 76.3,84.4 93.7,114.4 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 76.3,24.3 93.7,54.3 111.0,84.4 128.4,114.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:520;--dur:4.2s" points="24.3,54.3 59.0,54.3 76.3,24.3 41.6,24.3 24.3,54.3 41.6,84.4 76.3,84.4 93.7,114.4 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 76.3,24.3 93.7,54.3 111.0,84.4 128.4,114.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="54.3" x2="35.4" y2="54.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,54.3 35.4,58.1 35.4,50.5" fill="#fc77be"/><circle cx="24.3" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqeqqqwqaww` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `list of identifiers` ([[List]], [[Identifier]])
- **ID:** `hexpose:get_blockstates`

```text
before   … │ vector               ◂ top
after    … │ list of identifiers  ◂ top
```

## Description

Pushes a list of all facets of that block.

## Related patterns (same book page)

- [[Miner's Purification]] `identifier → number`
- [[Demoman's Purification]] `identifier → number`
- [[Orientation Purification]] `vector → vector/null`
- [[Farmer's Purification]] `vector → number/null`
- [[Facet Distillation]] `vector, identifier → any`
- [[Cartographer's Purification]] `identifier → vector`

## Sources

- 📖 **Guidebook:** [[Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Facet Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/blocks@hexpose:get_blockstates)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetBlockProperties` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_blockstates`
