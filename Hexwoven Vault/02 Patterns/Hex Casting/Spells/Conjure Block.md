---
aliases: ["Conjure Block"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hex Casting"
op_id: "hexcasting:conjure_block"
signature: "qqa"
start_direction: "NORTH_EAST"
input: "vector"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Block Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Block

✦ **Spell** · [[Hex Casting]] · 📖 [[Block Manipulation]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,132.0 119.4,85.0 92.3,38.0 38.0,38.0 65.1,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="92.3,132.0 119.4,85.0 92.3,38.0 38.0,38.0 65.1,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="132.0" x2="101.0" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.2,106.2 106.1,119.9 95.8,114.0" fill="#fc77be"/><circle cx="92.3" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexcasting:conjure_block`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Conjure an ethereal, but solid, block that sparkles with my [[Pigments|pigment]] at the given position. Costs about one [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Place Block]] `vector → —`
- [[Break Block]] `vector → —`
- [[Create Water]] `vector → —`
- [[Destroy Liquid]] `vector → —`
- [[Conjure Light]] `vector → —`
- [[Overgrow]] `vector → —`
- [[Edify Sapling]] `vector → —`
- [[Ignite]] `entity | vector → —`
- [[Extinguish Area]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Block Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Block](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/blockworks@hexcasting:conjure_block)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:conjure_block`
