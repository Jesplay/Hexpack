---
aliases: ["Cook"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:smelt"
signature: "qwqqadadadewewewe"
start_direction: "SOUTH_EAST"
input: "item entity"
output: ""
cost: "Costs about one Amethyst Dust for every ten seconds that the Furnace would need to have been active."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Block Mimicry]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cook

✦ **Spell** · [[Hexical]] · 📖 [[Block Mimicry]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 116.5,76.6 100.7,103.8 85.0,76.6 69.3,103.8 53.5,76.6 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 116.5,76.6 100.7,103.8 85.0,76.6 69.3,103.8 53.5,76.6 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="103.8" x2="42.8" y2="112.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.4,118.8 39.8,114.3 45.8,110.8" fill="#fc77be"/><circle cx="37.8" cy="103.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwqqadadadewewewe` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust for every ten seconds that the Furnace would need to have been active.
- **ID:** `hexical:smelt`

```text
before   … │ item entity  ◂ top
after    …                ◂ top
```

## Description

Takes an item entity and cooks it as if in a **Furnace**. Costs about one [[Amethyst|Amethyst Dust]] for every ten seconds that the **Furnace** would need to have been active.

## Related patterns (same book page)

- [[Push]] `vector, vector → —`
- [[Dispense]] `item entity, vector, vector → —`
- [[Roast]] `item entity → —`
- [[Smoke]] `item entity → —`
- [[Blast]] `item entity → —`
- [[Cut Stone]] `item entity, identifier → —`

## Sources

- 📖 **Guidebook:** [[Block Mimicry]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Cook](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/block_mimicry@hexical:smelt)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpCook` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:smelt`
