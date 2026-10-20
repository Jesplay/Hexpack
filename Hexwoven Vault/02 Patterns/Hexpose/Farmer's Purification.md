---
aliases: ["Farmer's Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:blockstate_crop"
signature: "qaqqqqqwaea"
start_direction: "EAST"
input: "vector"
output: "number/null"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Number]]", "[[Null]]"]
entry: "[[Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Farmer's Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Blocks]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,103.4 93.5,103.4 114.8,66.6 72.3,66.6 51.0,103.4 72.3,140.2 114.8,140.2 136.1,103.4 114.8,66.6 93.5,29.8 72.3,66.6 29.8,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:510;--dur:3.4s" points="51.0,103.4 93.5,103.4 114.8,66.6 72.3,66.6 51.0,103.4 72.3,140.2 114.8,140.2 136.1,103.4 114.8,66.6 93.5,29.8 72.3,66.6 29.8,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="103.4" x2="64.6" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,103.4 64.6,108.1 64.6,98.7" fill="#fc77be"/><circle cx="51.0" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqqwaea` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `number/null` ([[Number]], [[Null]])
- **ID:** `hexpose:blockstate_crop`

```text
before   … │ vector       ◂ top
after    … │ number/null  ◂ top
```

## Description

Pushes a number 0 to 1 corresponding to the growth progress of that block. Works for crops, **Beehives**, **Composters**, **Cauldrons**, and strangely **Cakes** too.

Every block has a list of *facets*: simple aspects that distinguish them from other blocks of the same type. For example, orientation, how far along a crop is to growing, and whether a door is open or shut are all facets.

Nature provides two patterns that allow **Hexcasting** to read and analyze facets of any blocks within ambit.

## Related patterns (same book page)

- [[Miner's Purification]] `identifier → number`
- [[Demoman's Purification]] `identifier → number`
- [[Orientation Purification]] `vector → vector/null`
- [[Facet Purification]] `vector → list of identifiers`
- [[Facet Distillation]] `vector, identifier → any`
- [[Cartographer's Purification]] `identifier → vector`

## Sources

- 📖 **Guidebook:** [[Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Farmer's Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/blocks@hexpose:blockstate_crop)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetBlockStateData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:blockstate_crop`
