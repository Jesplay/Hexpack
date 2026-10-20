---
aliases: ["Conjure Flower"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:flower"
signature: "weqqqqqwaeaeaeaeaea"
start_direction: "NORTH_EAST"
input: "vector, identifer"
output: ""
cost: "Costs about a quarter of an Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Identifier]]"]
iotas_out: []
entry: "[[Hexical Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Conjure Flower

✦ **Spell** · [[Hexical]] · 📖 [[Hexical Spells]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="131" height="170" viewBox="0 0 130.5 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="20.8,149.2 35.6,123.5 50.4,97.8 80.1,97.8 94.9,72.2 80.1,46.5 50.4,46.5 35.6,72.2 50.4,97.8 65.3,123.5 80.1,97.8 109.8,97.8 94.9,72.2 109.8,46.5 80.1,46.5 65.3,20.8 50.4,46.5 20.8,46.5 35.6,72.2 20.8,97.8 50.4,97.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:593;--dur:5.6s" points="20.8,149.2 35.6,123.5 50.4,97.8 80.1,97.8 94.9,72.2 80.1,46.5 50.4,46.5 35.6,72.2 50.4,97.8 65.3,123.5 80.1,97.8 109.8,97.8 94.9,72.2 109.8,46.5 80.1,46.5 65.3,20.8 50.4,46.5 20.8,46.5 35.6,72.2 20.8,97.8 50.4,97.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.8" y1="149.2" x2="25.5" y2="141.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="28.9,135.1 28.3,142.6 22.7,139.4" fill="#fc77be"/><circle cx="20.8" cy="149.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weqqqqqwaeaeaeaeaea` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, identifer` ([[Vector]], [[Identifier]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a quarter of an Amethyst Dust.
- **ID:** `hexical:flower`

```text
before   … │ vector │ identifer  ◂ top
after    …                       ◂ top
```

## Description

Conjures a flower of my choosing at a location. I must have identified this flower at some point before. Costs about a quarter of an [[Amethyst|Amethyst Dust]].

The block under the target position must have a flat top surface to allow rooting. It does not necessarily need to be able to support the flower naturally, although any little movement may cause the flower to break if conjured on an incompatible block. Alternatively, the spell will fill a flower pot if possible.

*There was a fool who praised me for the magic I acquired. That's all.*

## Related patterns (same book page)

- [[Confetti]] `vector, number/vector → —`
- [[Vibrate]] `vector, entity/vector, number → —`
- [[Sparkle]] `vector, vector, number → —`
- [[Crack Device]] `— → —`
- [[Illuminate]] `vector, number → —`
- [[Gasp]] `entity → —`
- [[Squawk]] `vector, identifier → —`

## Sources

- 📖 **Guidebook:** [[Hexical Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Flower](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/hexical_spells@hexical:flower)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureFlower` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:flower`
