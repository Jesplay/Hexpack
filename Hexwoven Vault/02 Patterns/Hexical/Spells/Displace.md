---
aliases: ["Displace"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/circles]
kind: "Spell"
family: "Spell"
topics: ["Spell Circles, Loci & Engines"]
mod: "Hexical"
op_id: "hexical:displace"
signature: "qaqqqqeedaqqqa"
start_direction: "NORTH_EAST"
input: "entity, vector"
output: ""
cost: "Costs about half of an Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Circle Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Displace

✦ **Spell** · [[Hexical]] · 📖 [[Circle Spells]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/><circle cx="22.0" cy="103.8" r="2"/><circle cx="53.5" cy="103.8" r="2"/><circle cx="85.0" cy="103.8" r="2"/><circle cx="116.5" cy="103.8" r="2"/><circle cx="148.0" cy="103.8" r="2"/><circle cx="6.3" cy="131.1" r="2"/><circle cx="37.8" cy="131.1" r="2"/><circle cx="69.3" cy="131.1" r="2"/><circle cx="100.7" cy="131.1" r="2"/><circle cx="132.2" cy="131.1" r="2"/><circle cx="163.7" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="37.8,131.1 53.5,103.8 37.8,76.6 22.0,103.8 37.8,131.1 69.3,131.1 85.0,103.8 69.3,76.6 85.0,49.3 116.5,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 116.5,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="37.8,131.1 53.5,103.8 37.8,76.6 22.0,103.8 37.8,131.1 69.3,131.1 85.0,103.8 69.3,76.6 85.0,49.3 116.5,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 116.5,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="131.1" x2="42.8" y2="122.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.4,116.1 45.8,124.1 39.8,120.6" fill="#fc77be"/><circle cx="37.8" cy="131.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqeedaqqqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about half of an Amethyst Dust.
- **ID:** `hexical:displace`

```text
before   … │ entity │ vector  ◂ top
after    …                    ◂ top
```

## Description

Teleports an entity within the [[Spell Circles|circle]] to another point within the same [[Spell Circles|circle]]. Unlike [[Greater Teleport (Book Entry)|Greater Teleport]], this spell takes in world coordinates rather than an offset. Costs about half of an [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Appendage]] `vector → —`
- [[Export Media]] `vector, number → —`

## Sources

- 📖 **Guidebook:** [[Circle Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Displace](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/circle_spells@hexical:displace)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpDisplace` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:displace`
