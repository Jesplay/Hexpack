---
aliases: ["Facet Distillation"]
tags: [hexcasting/pattern, mod/hexpose, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:query_blockstate"
signature: "qaqqqqqeawa"
start_direction: "EAST"
input: "vector, identifier"
output: "any"
cost: ""
iotas_in: ["[[Vector]]", "[[Identifier]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Facet Distillation

◇ **Pattern** · [[Hexpose]] · 📖 [[Blocks]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,103.4 72.3,103.4 93.5,66.6 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:510;--dur:3.4s" points="29.8,103.4 72.3,103.4 93.5,66.6 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="103.4" x2="43.4" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="53.2,103.4 43.4,108.1 43.4,98.7" fill="#fc77be"/><circle cx="29.8" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqqeawa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, identifier` ([[Vector]], [[Identifier]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexpose:query_blockstate`

```text
before   … │ vector │ identifier  ◂ top
after    … │ any                  ◂ top
```

## Description

Takes a block and a facet and pushes that facet's value.

If the block does not have the given facet, the pattern pushes Null. For the most part, it is quite intuitive: the pattern produces booleans, directional vectors, and integers, whichever matches the facet best.

For certain facets that deal with a state among a variety of them, it will produce a seemingly random yet consistent integer. If a facet is too alien, a senseless [[Iotas|iota]] is produced.

## Related patterns (same book page)

- [[Miner's Purification]] `identifier → number`
- [[Demoman's Purification]] `identifier → number`
- [[Orientation Purification]] `vector → vector/null`
- [[Farmer's Purification]] `vector → number/null`
- [[Facet Purification]] `vector → list of identifiers`
- [[Cartographer's Purification]] `identifier → vector`

## Sources

- 📖 **Guidebook:** [[Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Facet Distillation](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/blocks@hexpose:query_blockstate)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpQueryBlockProperty` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:query_blockstate`
