---
aliases: ["Destroy Liquid"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hex Casting"
op_id: "hexcasting:destroy_water"
signature: "dedwedade"
start_direction: "SOUTH_WEST"
input: "vector"
output: ""
cost: "Costs about two Charged Amethyst."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Block Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Destroy Liquid

✦ **Spell** · [[Hex Casting]] · 📖 [[Block Manipulation]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,66.6 72.3,103.4 51.0,66.6 72.3,29.8 93.5,66.6 114.8,103.4 93.5,140.2 72.3,103.4 51.0,140.2 29.8,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="93.5,66.6 72.3,103.4 51.0,66.6 72.3,29.8 93.5,66.6 114.8,103.4 93.5,140.2 72.3,103.4 51.0,140.2 29.8,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="66.6" x2="86.7" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="81.9,86.8 82.7,76.0 90.8,80.7" fill="#fc77be"/><circle cx="93.5" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedwedade` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about two Charged Amethyst.
- **ID:** `hexcasting:destroy_water`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Drains either a liquid container at, or a body of liquid around, the given position. Costs about two [[Amethyst|Charged Amethyst]].

## Related patterns (same book page)

- [[Place Block]] `vector → —`
- [[Break Block]] `vector → —`
- [[Create Water]] `vector → —`
- [[Conjure Block]] `vector → —`
- [[Conjure Light]] `vector → —`
- [[Overgrow]] `vector → —`
- [[Edify Sapling]] `vector → —`
- [[Ignite]] `entity | vector → —`
- [[Extinguish Area]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Block Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Destroy Liquid](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/blockworks@hexcasting:destroy_water)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpDestroyFluid` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:destroy_water`
