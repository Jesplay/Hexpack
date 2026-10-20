---
aliases: ["Overgrow"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hex Casting"
op_id: "hexcasting:bonemeal"
signature: "wqaqwawqaqw"
start_direction: "NORTH_EAST"
input: "vector"
output: ""
cost: "Costs a bit more than one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Block Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Overgrow

✦ **Spell** · [[Hex Casting]] · 📖 [[Block Manipulation]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 72.3,140.2 93.5,103.4 114.8,66.6 93.5,29.8 72.3,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:510;--dur:3.4s" points="29.8,140.2 51.0,103.4 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 72.3,140.2 93.5,103.4 114.8,66.6 93.5,29.8 72.3,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaqwawqaqw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a bit more than one Amethyst Dust.
- **ID:** `hexcasting:bonemeal`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Encourage a plant or sapling at the target position to grow, as if **Bonemeal** was applied. Costs a bit more than one [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Place Block]] `vector → —`
- [[Break Block]] `vector → —`
- [[Create Water]] `vector → —`
- [[Destroy Liquid]] `vector → —`
- [[Conjure Block]] `vector → —`
- [[Conjure Light]] `vector → —`
- [[Edify Sapling]] `vector → —`
- [[Ignite]] `entity | vector → —`
- [[Extinguish Area]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Block Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Overgrow](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/blockworks@hexcasting:bonemeal)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpTheOnlyReasonAnyoneDownloadedPsi` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:bonemeal`
