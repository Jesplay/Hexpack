---
aliases: ["Falling Block"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexal"
op_id: "hexal:falling_block"
signature: "wqwawqwqwqwqwqw"
start_direction: "EAST"
input: "vec"
output: ""
cost: "Costs one and a half Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Hexal Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Falling Block

✦ **Spell** · [[Hexal]] · 📖 [[Hexal Spells]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="22.0,76.6 53.5,76.6 85.0,76.6 100.7,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.5s" points="22.0,76.6 53.5,76.6 85.0,76.6 100.7,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="76.6" x2="32.1" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.4,76.6 32.1,80.0 32.1,73.1" fill="#fc77be"/><circle cx="22.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwawqwqwqwqwqw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one and a half Amethyst Dust.
- **ID:** `hexal:falling_block`

```text
before   … │ vec  ◂ top
after    …        ◂ top
```

## Description

Remove a location from the [[The Stack|stack]], and force the block there to fall as if it were sand or gravel. Costs one and a half [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Smelt]] `vec | entity | mote → —`
- [[Freeze]] `vec → —`
- [[Place Block II]] `(itemtype | mote), vec → —`
- [[Particles]] `vec | [vec] → —`

## Sources

- 📖 **Guidebook:** [[Hexal Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Falling Block](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/hexal_spells@hexal:falling_block)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpFallingBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:falling_block`
